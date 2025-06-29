//! #FSM Crate for DH09
//!
//! The 'fsm' crate is used by Dh09 to keep track of the state in which the pod
//! is. Transitions between states are triggered by pre-determined events sent
//! from each subsystem.

#![no_std]
#![no_main]

mod entry_methods;
mod exit_methods;
pub(crate) mod fsm;

// TODO: Move to generated
/// Struct with all the subsystems checked during the system check
#[derive(Clone, Debug, Copy)]
pub struct CheckedSystems {
    /// Marks if powertrain has been checked
    powertrain: bool,
    /// Marks if levitation has been checked
    levitation: bool,
    /// Marks if propulsion has been checked
    propulsion: bool,
}

/// Enum with all the systems checked during the system check
#[derive(Copy, Debug, Clone)]
pub enum CheckedSystem {
    /// Used to indicate that powertrain has been checked
    Powertrain,
    /// Used to indicate that levitation has been checked
    Levitation,
    /// Used to indicate that propulsion has been checked
    Propulsion,
}

pub use fsm::FSM;
pub use lib::utils::data::Event;
pub use lib::States;
