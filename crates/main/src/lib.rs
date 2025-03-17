//! Lib code.

#![no_std]

pub mod can;
pub mod gs_master;

pub mod config {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}
