use std::sync::Mutex;
use std::time::Duration;
use std::io::{stdout, Write};
use std::str::FromStr;
use gslib::{Datatype, Message, Datapoint};
use gslib::ERROR_CHANNEL;
use gslib::HEARTBEAT;
use gslib::INFO_CHANNEL;
use gslib::SHORTCUT_CHANNEL;
use gslib::STATUS_CHANNEL;
use gslib::WARNING_CHANNEL;
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
use tauri::Manager;
use tokio::time::sleep;

use crate::backend::Backend;
use crate::frontend::commands::*;
use crate::frontend::BackendState;
use crate::frontend::BACKEND;
use crate::frontend::datapoint_dict::DatapointDict;

pub static APP_HANDLE: Mutex<Option<&AppHandle>> = Mutex::new(None);

pub fn tauri_main(backend: Backend) {
    println!("Starting tauri application");
    tauri::Builder::default()
        .manage(BackendState::default())
        .invoke_handler(tauri::generate_handler![
            unload_buffer,
            send_command,
            send_command_64_bits,
            generate_test_data,
            connect_to_pod,
            disconnect,
            procedures,
            test_panic,
            save_logs,
        ])
        .plugin(
            tauri_plugin_global_shortcut::Builder::new().with_handler(|app, shortcut, _event| {
                if shortcut.matches(Modifiers::empty(), Code::Space) || shortcut.matches(Modifiers::empty(), Code::Escape) {
                    send_command("EmergencyBrake".into(), 0);
                    app.emit(STATUS_CHANNEL, "Emergency Brake triggered!;red").unwrap();
                    app.emit(ERROR_CHANNEL, "Emergency Brake triggered!").unwrap()
                } else if shortcut.matches(Modifiers::empty(), Code::KeyL) {
                    app.emit(SHORTCUT_CHANNEL, "ToggleLogs").unwrap();
                } else if shortcut.matches(Modifiers::empty(), Code::KeyD) {
                    app.emit(SHORTCUT_CHANNEL, "DebugMode").unwrap();
                } else if shortcut.matches(Modifiers::empty(), Code::KeyM) {
                    app.emit(SHORTCUT_CHANNEL, "OpenMenu").unwrap();
                } else {
                    // Tabs will also go here
                    app.emit(SHORTCUT_CHANNEL, shortcut.into_string()).unwrap();
                }
            })
                .build(),
        )
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();

            let mut message_rcv = backend.message_receiver.resubscribe();
            unsafe {
                BACKEND.replace(Mutex::new(backend));
            }

            // this is unsafe, don't do it anywhere else
            // APP_HANDLE
            //     .lock()
            //     .map(|mut x| x.deref_mut().replace(app))
            //     .expect("Error replacing app handle mutex");
            
            // set up heartbeat
            let app_clone = app.handle().clone();
            tokio::spawn(async move {
                loop {
                    app_clone.emit(SHORTCUT_CHANNEL, "heartbeat").unwrap();
                    sleep(Duration::from_millis(HEARTBEAT)).await;
                }
            });

            // Shortcuts
            let sh = app.global_shortcut();

            sh.register("Space").expect("Could not register shortcut");
            sh.register("Esc").expect("Could not register shortcut");
            sh.register("L").expect("Could not register shortcut");
            sh.register("D").expect("Could not register shortcut");
            sh.register("M").expect("Could not register shortcut");

            for i in 1..10 {
                sh.register(
                    Shortcut::new(
                        Option::from(Modifiers::SHIFT),
                        Code::from_str(&format!("Digit{}", i)).unwrap()
                    )
                ).expect("Could not register shortcut");
            }

            // --

            let app_clone = app.handle().clone();
            tokio::spawn(async move {
                let capacity = 50;
                let mut datapoint_dict: DatapointDict = DatapointDict::new(capacity);
                print!("{}", "\n".repeat(capacity + 10));
                loop {
                    match message_rcv.try_recv() {
                        Ok(msg) => {
                            if let Some(backend_mutex) = unsafe { BACKEND.as_mut() } {
                                backend_mutex.get_mut().unwrap().log_msg(&msg);
                            }

                            match msg {
                                Message::Data(dp) => {
                //                     // println!("Received datapoint: {:?}", dp);
                                    if dp.datatype == Datatype::CANLog {
                                        app_clone.emit(INFO_CHANNEL, format!("Received datapoint on the main PCB: {:?}", dp)).expect("Couldn't send message");
                                    }
                                    datapoint_dict.add_datapoint(Datapoint::new(dp.datatype, dp.value as u64, dp.timestamp));
                                    print!("{}", datapoint_dict);
                                    stdout().flush().unwrap();
                                    app_clone
                                        .state::<BackendState>()
                                        .data_buffer
                                        .lock()
                                        .unwrap()
                                        .push(Message::Data(dp));
                                },
                                Message::Status(s) => app_clone
                                    .emit(
                                        STATUS_CHANNEL,
                                        &*format!("Status: {:?};{}", s, s.to_colour_str()),
                                    )
                                    .unwrap(),
                                Message::Info(i) => {
                                    app_clone.emit(INFO_CHANNEL, i.to_string()).unwrap()
                                },
                                Message::Warning(w) => {
                                    app_clone.emit(WARNING_CHANNEL, w.to_string()).unwrap()
                                },
                                Message::Error(e) => {
                                    app_clone.emit(ERROR_CHANNEL, e.to_string()).unwrap()
                                },
                            }
                        },
                        Err(_e) => {
                            // eprintln!("Error receiving message: {:?}", e);
                        },
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
