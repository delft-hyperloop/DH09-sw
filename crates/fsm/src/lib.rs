//! #FSM Crate for DH09
//!
//! The 'fsm' crate is used by Dh09 to keep track of the state in which the pod
//! is. Transitions between states are triggered by pre-determined events sent
//! from each subsystem.

#![no_std]
#![no_main]

pub(crate) mod fsm;

// TODO: Move to generated
/// Struct with all the subsystems checked during the system check
#[derive(Clone, Debug, Copy)]
pub struct CheckedSystems {
    /// Marks if levitation has been checked
    levitation: bool,
    /// Marks if propulsion motor 1 has been checked
    propulsion1: bool,
    /// Marks if propulsion motor 2 has been checked
    propulsion2: bool,
}

/// Enum with all the systems checked during the system check
#[derive(Copy, Debug, Clone)]
pub enum CheckedSystem {
    /// Used to indicate that levitation has been checked
    Levitation,
    /// Used to indicate that propulsion motor 1 has been checked
    Propulsion1,
    /// Used to indicate that propulsion motor 2 has been checked
    Propulsion2,
}

pub use fsm::FSM;
pub use lib::utils::data::Event;
pub use lib::States;
