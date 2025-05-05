//! Lib code.

#![no_std]

/// Module that contains the functionality for sending and receiving messages on the two CAN busses
pub mod can;
pub mod gs_master;

pub mod config {
    #![allow(missing_docs, unused)]
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}
