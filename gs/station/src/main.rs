// Prevents an additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::absolute_paths)]

use gslib::Command;
use gslib::Message;

use crate::backend::Backend;
#[cfg(feature = "gui")]
use crate::frontend::app::tauri_main;

mod backend;
pub mod connect;
mod data;
#[cfg(feature = "gui")]
mod frontend;

pub type CommandSender = tokio::sync::broadcast::Sender<Command>;
pub type CommandReceiver = tokio::sync::broadcast::Receiver<Command>;
pub type MessageSender = tokio::sync::broadcast::Sender<Message>;
pub type MessageReceiver = tokio::sync::broadcast::Receiver<Message>;

/// Entry point of the application
#[tokio::main]
async fn main() {
    let backend = Backend::new();

    #[cfg(feature = "gui")]
    tauri_main(backend);

    if !cfg!(feature = "gui") {
        println!("No features enabled, exiting");
    }
}
