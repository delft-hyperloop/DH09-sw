//! #FSM Crate for DH09
//!
//! The 'fsm' crate is used by Dh09 to keep track of the state in which the pod
//! is. Transitions between states are triggered by pre-determined events sent
//! from each subsystem.

#![no_std]
#![no_main]

pub(crate) mod fsm;
pub mod utils;
mod entry_methods;
mod exit_methods;

#[cfg(test)]
mod tests;

pub use fsm::States;
pub use fsm::FSM;
