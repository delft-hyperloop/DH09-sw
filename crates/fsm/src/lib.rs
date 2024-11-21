//! #FSM Crate for DH09
//!
//! The 'fsm' crate is used by Dh09 to keep track of the state in which the pod is.
//! It's built on the principle of state charts, as it has one superstate (the "Operating" state)
//! that runs multiple sub-FSMs that keep track of the subsystems that run during normal operation of the pod.
//! The transitions are triggered by pre-determined events sent from each subsystem.

#![no_std]
#![no_main]
extern crate alloc;

pub mod commons;
mod high_voltage_fsm;
mod emergency_fsm;
mod operating_fsm;
mod propulsion_fsm;
mod levitation_fsm;

use alloc::sync::Arc;
use core::cmp::PartialEq;
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex};
use embassy_sync::signal::Signal;
use commons::Event;
use MainStates::*;
use crate::commons::{EmergencyChannel, EventChannel, PriorityEventPubSub, Runner, Transition};
use crate::emergency_fsm::{EmergencyFSM};
use crate::high_voltage_fsm::{HighVoltageFSM};
use crate::levitation_fsm::{LevitationFSM};
use crate::operating_fsm::{OperatingFSM};
use crate::propulsion_fsm::{PropulsionFSM};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum MainStates {
    SystemCheck = 0,
    Idle,
    Charging,
    Active,
    FlashingCode,
    Operating,
}

pub struct MainFSM {
    state: MainStates,
    // peripherals: // TODO: add peripherals
    priority_event_pub_sub: Arc<PriorityEventPubSub>,
}

static RUN_SUB_FSM: Signal<CriticalSectionRawMutex, bool> = Signal::new();

pub(crate) static HIGH_VOLTAGE_STATE: AtomicBool = AtomicBool::new(false);
pub(crate) static LEVITATION_STATE: AtomicBool = AtomicBool::new(false);
pub(crate) static PROPULSION_STATE: AtomicBool = AtomicBool::new(false);
pub(crate) static EMERGENCY_STATE: AtomicBool = AtomicBool::new(false);

impl MainFSM {
    pub fn new(
        spawner: Spawner,
        // peripherals: // TODO: add peripherals
        event_channel: &'static EventChannel,
        emergency_channel: &'static EmergencyChannel,
    ) -> Self {
        let high_voltage_fsm = define_fsm!(HighVoltageFSM, event_channel, emergency_channel);
        let emergency_fsm = define_fsm!(EmergencyFSM, event_channel, emergency_channel);
        let operating_fsm = define_fsm!(OperatingFSM, event_channel, emergency_channel);
        let propulsion_fsm = define_fsm!(PropulsionFSM, event_channel, emergency_channel);
        let levitation_fsm = define_fsm!(LevitationFSM, event_channel, emergency_channel);

        spawner.spawn(run_high_voltage_fsm(high_voltage_fsm)).unwrap();
        spawner.spawn(run_emergency_fsm(emergency_fsm)).unwrap();
        spawner.spawn(run_operating_fsm(operating_fsm)).unwrap();
        spawner.spawn(run_propulsion_fsm(propulsion_fsm)).unwrap();
        spawner.spawn(run_levitation_fsm(levitation_fsm)).unwrap();

        Self {
            state: SystemCheck,
            priority_event_pub_sub: Arc::new(PriorityEventPubSub::new(
                event_channel.publisher().unwrap(),
                event_channel.subscriber().unwrap(),
                emergency_channel.publisher().unwrap(),
                emergency_channel.subscriber().unwrap(),
            )),
        }
    }


    async fn handle(&mut self, event: Event) -> bool {
        match (&self.state, event) {
            (_, Event::Emergency) => {
                // TODO: attempt shut down
                // TODO: transition to quit
            }
            (SystemCheck, Event::SystemCheckSuccess) => self.transition(Idle, None),
            (Idle, Event::Activate) => self.transition(Active, None),
            (Idle, Event::Charge) => self.transition(Charging, None),
            (Charging, Event::StopCharge) => self.transition(Idle, None),
            (Active, Event::Operate) => {
                RUN_SUB_FSM.signal(true);
                self.transition(Operating, None);
            },
            (Operating, Event::Quit) => {
                // TODO: add checks for propulsion, levitation, hv
                return false;
            }
            _ => {}
        }
        true
    }
}

#[embassy_executor::task]
pub async fn run_propulsion_fsm(mut propulsion_fsm: PropulsionFSM) {
    RUN_SUB_FSM.wait().await;
    propulsion_fsm.run();
}

#[embassy_executor::task]
pub async fn run_levitation_fsm(mut levitation_fsm: LevitationFSM) {
    RUN_SUB_FSM.wait().await;
    levitation_fsm.run();
}

#[embassy_executor::task]
pub async fn run_high_voltage_fsm(mut high_voltage_fsm: HighVoltageFSM) {
    RUN_SUB_FSM.wait().await;
    high_voltage_fsm.run();
}

#[embassy_executor::task]
pub async fn run_operating_fsm(mut operating_fsm: OperatingFSM) {
    RUN_SUB_FSM.wait().await;
    operating_fsm.run();
}

#[embassy_executor::task]
pub async fn run_emergency_fsm(mut emergency_fsm: EmergencyFSM) {
    RUN_SUB_FSM.wait().await;
    emergency_fsm.run();
}

impl_runner_get_sub_channel!(MainFSM);
impl_transition!(MainFSM, MainStates);

static ENTRY_FUNCTION_MAP: [fn(); 6] = [
    || (),  // SystemCheck
    || (),  // Idle
    || (),  // Charging
    enter_active,
    || (),  // FlashingCode
    || (),  // Operating
];

static EXIT_FUNCTION_MAP: [fn(); 6] = [
    || (),  // SystemCheck
    || (),  // Idle
    || (),  // Charging
    || (),  // Active
    || (),  // FlashingCode
    || (),  // Operating
];

fn enter_active() {
    // TODO: Send CAN command to turn on high voltage
    HIGH_VOLTAGE_STATE.store(true, Ordering::Relaxed);
    // TODO: Close SDC while keeping brakes engaged
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_basic_transition() {
//         let mut spawner =
//         let mut event_channel = EventChannel::new();
//         let mut fsm = MainFSM::new(spawner, event_channel);
//
//         fsm.run();
//
//         assert_eq!(fsm.state, SystemCheck);
//     }
//
//     #[test]
//     fn test_multiple_events() {
//         // TODO
//     }
//
//     #[test]
//     fn test_invalid_event_order() {
//         // TODO
//     }
// }