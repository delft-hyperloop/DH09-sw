#![no_std]

pub mod can;
pub mod utils;

pub mod config {
    #![allow(
        missing_docs,
        unused,
        missing_copy_implementations,
        missing_debug_implementations,
        clippy::match_single_binding,
        clippy::needless_range_loop,
    )]
    #![deny(clippy::match_overlapping_arm)]
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

// export these so they're visible under `lib::`
pub use utils::data::EmergencyType;
pub use utils::data::Event;
pub use utils::datapoint::Datapoint;
pub use utils::event_types::EventChannel;
pub use utils::event_types::EventReceiver;
pub use utils::event_types::EventSender;

pub use crate::config::States;
