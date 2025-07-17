use std::path::PathBuf;
use std::str::FromStr;

use chrono::Local;
use gslib::Datapoint;
use gslib::Datatype;
use gslib::Limit;
use gslib::Message;
use gslib::ProcessedData;
use gslib::States;
use rand::Rng;
use tauri::Manager;
use tauri::State;

use crate::backend::Backend;
use crate::frontend::app::APP_HANDLE;
use crate::frontend::BackendState;
use crate::frontend::BACKEND;
use crate::Command;

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn generate_test_data() -> Vec<Datapoint> {
    let mut rng = rand::thread_rng();
    let mut datapoints = Vec::new();

    let value: u64 = rng.gen_range(0..101);
    let value2: u64 = rng.gen_range(0..101);
    let value3: u64 = rng.gen_range(0..101);
    let value4: u64 = rng.gen_range(0..300);

    let datapoint = Datapoint { value, datatype: Datatype::from_id(1200), timestamp: 0 };
    let datapoint2 = Datapoint { value: value2, datatype: Datatype::from_id(1200), timestamp: 0 };
    let datapoint3 = Datapoint { value: 1, datatype: Datatype::from_id(1200), timestamp: 0 };
    let datapoint4 = Datapoint { value: 2, datatype: Datatype::from_id(1200), timestamp: 0 };
    let datapoint5 = Datapoint { value: 3, datatype: Datatype::from_id(1200), timestamp: 0 };
    let datapoint6 =
        Datapoint { value: value4, datatype: Datatype::BMSTemperatureHigh, timestamp: 0 };
    let datapoint7 =
        Datapoint { value: value4, datatype: Datatype::BMSTemperatureLow, timestamp: 0 };

    datapoints.push(datapoint);
    datapoints.push(datapoint2);
    datapoints.push(datapoint3);
    datapoints.push(datapoint4);
    datapoints.push(datapoint5);
    datapoints.push(datapoint6);
    datapoints.push(datapoint7);

    datapoints
}

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn get_fsm_state_by_index(index: u8) -> String { format!("{:?}", States::from_index(index)) }

#[tauri::command]
pub fn get_datatype_by_id(id: u16) -> String { format!("{:?}", Datatype::from_id(id)) }

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn get_unit_by_datatype(datatype: String) -> String {
    Datatype::from_str(&datatype).unit().to_string()
}

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn get_ranges_by_datatype_id(datatype: String) -> String {
    let bounds = Datatype::from_str(&datatype).bounds();
    match bounds {
        (Limit::Single(upper), Limit::Single(lower)) => format!("[{lower}, {upper}]"),
        (Limit::Multiple(severities_upper), Limit::Multiple(severities_lower)) => {
            if severities_lower.brake.is_some() && severities_upper.brake.is_some() {
                return format!(
                    "[{}, {}]",
                    severities_lower.brake.unwrap(),
                    severities_upper.brake.unwrap()
                );
            }
            "".to_string()
        },
        _ => "".to_string(),
    }
}

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn unload_buffer(state: State<BackendState>) -> Vec<ProcessedData> {
    let mut data_buffer = state.data_buffer.lock().unwrap();
    let mut datapoints = Vec::new();
    for msg in data_buffer.iter() {
        if let Message::Data(datapoint) = msg {
            datapoints.push(datapoint.clone());
        }
    }
    data_buffer.clear();
    datapoints
}

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn send_command(cmd_name: String, val: u64) -> bool {
    if cmd_name != "FrontendHeartbeat" {
        eprintln!("[frontend] Sending command {}({}) [{}]", cmd_name, val, Local::now());
    }
    let c = Command::from_string(&cmd_name, val);
    if let Ok(mut backend_mutex) = BACKEND.lock() {
        unsafe { backend_mutex.assume_init_mut().send_command(c) }
    } else {
        panic!("kys");
    }
}

fn i32_to_u64(x: [i32; 2]) -> u64 {
    let a = i32::to_be_bytes(x[0]);
    let b = i32::to_be_bytes(x[1]);
    u64::from_be_bytes([a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3]])
}

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn send_command_64_bits(cmd_name: String, vals: [i32; 2]) -> bool {
    let value = i32_to_u64(vals);
    if cmd_name != "FrontendHeartbeat" {
        eprintln!("[frontend] Sending command {}({}) [{}]", cmd_name, value, Local::now());
    }
    let c = Command::from_string(&cmd_name, value);
    if let Ok(mut backend_mutex) = BACKEND.lock() {
        unsafe { backend_mutex.assume_init_mut().send_command(c) }
    } else {
        panic!("kys");
    }
}

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn connect_to_pod() -> bool {
    if let Ok(mut backend_mutex) = BACKEND.lock() {
        unsafe { backend_mutex.assume_init_mut().start_server() }
    } else {
        false
    }
}

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn disconnect() -> bool {
    if let Ok(mut backend_mutex) = BACKEND.lock() {
        unsafe {
            backend_mutex.assume_init_mut().quit_server();
        }
        true
    } else {
        false
    }
}

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn save_logs() -> bool {
    if let Ok(mut backend_mutex) = BACKEND.lock() {
        let mut b = unsafe { backend_mutex.assume_init_mut() };
        
        if Backend::save_to_path(&mut b.log).is_ok() {
            if let Ok(Some(app)) = APP_HANDLE.try_lock().map(|lock| lock.clone()) {
                let _ = app.emit_all("clear_logs", "Logs saved");
            }
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn save_to_file(path: &str) -> bool {
    false
    // if let Ok(mut backend_mutex) = BACKEND.lock() {
    //     let log = unsafe { &backend_mutex.assume_init_mut().log };
    //     let Ok(x) = PathBuf::from_str(path);
    //     Backend::save_to_path(log, x).is_ok()
    // } else {
    //     false
    // }
}

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn procedures() -> Vec<[String; 6]> {
    let res = Backend::load_procedures(PathBuf::from("../../config/procedures/"));
    if let Ok(mut backend_mutex) = BACKEND.lock() {
        if let Ok(x) = res {
            unsafe {
                backend_mutex
                    .assume_init_mut()
                    .log_msg(&Message::Info("Loading procedures".into()));
            }
            x
        } else {
            unsafe {
                backend_mutex
                    .assume_init_mut()
                    .log_msg(&Message::Error("Failed to load procedures".into()));
            }
            vec![[
                "Failed".into(),
                "Failed to parse some procedures".into(),
                "".into(),
                "".into(),
                "".into(),
                format!("{res:?}"),
            ]]
        }
    } else {
        res.unwrap()
    }
}

#[macro_export]
#[allow(unused)]
#[tauri::command]
pub fn test_panic() {
    panic!("kill yourself");
}
