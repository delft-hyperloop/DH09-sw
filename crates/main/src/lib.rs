//! Lib code.

#![no_std]

/// Module that contains the functionality for sending and receiving messages on
/// the two CAN busses
pub mod can;
pub mod comms_forwarding_tasks;
pub mod gs_master;
pub mod ethernet;
