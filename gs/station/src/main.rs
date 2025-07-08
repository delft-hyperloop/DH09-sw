// Prevents an additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::absolute_paths)]

use gslib::Command;
use gslib::Message;

use crate::backend::Backend;
#[cfg(feature = "gui")]
use crate::frontend::app::tauri_main;
#[cfg(feature = "tui")]
use crate::tui::tui_main;

mod backend;
pub mod connect;
mod data;
#[cfg(feature = "gui")]
mod frontend;
#[cfg(feature = "tui")]
pub mod tui;

pub type CommandSender = tokio::sync::broadcast::Sender<Command>;
pub type CommandReceiver = tokio::sync::broadcast::Receiver<Command>;
pub type MessageSender = tokio::sync::broadcast::Sender<Message>;
pub type MessageReceiver = tokio::sync::broadcast::Receiver<Message>;

/// Entry point of the application
#[tokio::main]
async fn main() {
    let backend = Backend::new();

    // the tui is not a full frontend,
    // so it just needs a handle to receive data.
    #[cfg(feature = "tui")]
    let tui = {
        // // redirect stderr somewhere else
        // use std::fs::File;
        // use std::os::fd::AsRawFd;
        //
        // use crossterm::tty::IsTty;
        //
        // let log_file =
        //     File::create("/Users/andtsa/downloads/tauri.log").expect("couldn’t open log");
        // let logfd = log_file.try_clone().expect("??");
        // // SAFETY: questionable. replaces the global stderr handle
        // unsafe {
        //     libc::dup2(logfd.as_raw_fd(), libc::STDOUT_FILENO);
        //     libc::dup2(logfd.as_raw_fd(), libc::STDERR_FILENO);
        // }
        //
        // assert!(!std::io::stdout().is_tty());

        let rx = backend.message_receiver.resubscribe();
        tokio::spawn(async move { tui_main(rx).await })
    };

    #[cfg(feature = "gui")]
    tauri_main(backend);

    #[cfg(feature = "tui")]
    tui.await.unwrap().unwrap();

    if !cfg!(feature = "gui") && !cfg!(feature = "tui") {
        println!("No features enabled, exiting");
    }
}
