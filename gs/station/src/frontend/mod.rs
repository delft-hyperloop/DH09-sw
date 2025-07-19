pub mod app;
pub mod commands;
pub mod logging;

use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::sync::Mutex;

use gslib::Message;

use crate::backend::Backend;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BackendState {
    pub running: Mutex<bool>,
    pub data_buffer: Mutex<Vec<Message>>,
}

static BACKEND: Mutex<MaybeUninit<Backend>> = Mutex::new(MaybeUninit::uninit());

impl Default for BackendState {
    fn default() -> Self {
        Self { running: Mutex::new(false), data_buffer: Mutex::new(Vec::new()) }
    }
}
