#![no_std]

pub mod can1;
pub mod can2;
pub mod datapoint;
pub mod utils;

pub mod config {
    #![allow(
        missing_docs,
        unused,
        missing_copy_implementations,
        missing_debug_implementations
    )]
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

// export these so they're visible under `lib::`
pub use config::States;
pub use datapoint::Datapoint;
pub use utils::data::Event;
