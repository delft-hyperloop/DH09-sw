//! #FSM Crate for DH09
//!
//! The 'fsm' crate is used by Dh09 to keep track of the state in which the pod
//! is. Transitions between states are triggered by pre-determined events sent from each subsystem.

#![no_std]
#![no_main]

pub mod utils;
pub(crate) mod fsm;
// #[cfg(test)]
// mod tests;
mod entry_methods;
mod exit_methods;

pub use fsm::{FSM, States};
