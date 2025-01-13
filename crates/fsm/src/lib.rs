//! #FSM Crate for DH09
//!
//! The 'fsm' crate is used by Dh09 to keep track of the state in which the pod
//! is. It's built on the principle of state charts, as it has one superstate
//! (the "Operating" state) that runs multiple sub-FSMs that keep track of the
//! subsystems that run during normal operation of the pod. The transitions are
//! triggered by pre-determined events sent from each subsystem.

#![no_std]
#![no_main]

pub mod commons;
pub(crate) mod main_fsm;
pub(crate) mod emergency_fsm;
pub(crate) mod high_voltage_fsm;
pub(crate) mod levitation_fsm;
pub(crate) mod operating_fsm;
pub(crate) mod propulsion_fsm;
#[cfg(test)]
mod tests;

pub use main_fsm::{MainFSM, MainStates};
