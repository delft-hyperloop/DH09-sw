use std::thread;
use std::time::Duration;

use crate::backend::Backend;
use crate::frontend::BACKEND;

pub fn start_logging_watch() {
    thread::spawn(move || loop {
        if let Ok(mut backend_mutex) = BACKEND.lock() {
            let b = unsafe { backend_mutex.assume_init_mut() };

            if let Err(e) = Backend::save_to_path(&mut b.log) {
                eprintln!("error saving to file: {e}");
            }
        }
        thread::sleep(Duration::from_millis(1000));
    });
}
