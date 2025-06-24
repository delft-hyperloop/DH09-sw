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
    powertrain: bool,
    levitation: bool,
    propulsion: bool,
}

/// Enum with all the systems checked during the system check
pub enum CheckedSystem {
    Powertrain,
    Levitation,
    Propulsion
}

pub use fsm::FSM;
pub use lib::utils::data::Event;
pub use lib::States;
