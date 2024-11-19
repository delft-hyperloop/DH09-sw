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
use embassy_executor::Spawner;
use commons::Event;
use MainStates::*;
use crate::commons::{EmergencyChannel, EventChannel, PriorityEventPubSub, Runner, Transition};
use crate::emergency_fsm::EmergencyFSM;
use crate::high_voltage_fsm::{HighVoltageFSM};
use crate::levitation_fsm::LevitationFSM;
use crate::operating_fsm::OperatingFSM;
use crate::propulsion_fsm::PropulsionFSM;

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
    spawner: Spawner,
    state: MainStates,
    // peripherals: // TODO: add peripherals
    priority_event_pub_sub: Arc<PriorityEventPubSub>,
    high_voltage_fsm: HighVoltageFSM,
    emergency_fsm: EmergencyFSM,
    operating_fsm: OperatingFSM,
    propulsion_fsm: PropulsionFSM,
    levitation_fsm: LevitationFSM,
}

impl MainFSM {
    pub fn new(
        spawner: Spawner,
        // peripherals: // TODO: add peripherals
        event_channel: &'static EventChannel,
        emergency_channel: &'static EmergencyChannel,
    ) -> Self {
        let high_voltage_fsm = define_fsm!(HighVoltageFSM);
        let emergency_fsm = define_fsm!(EmergencyFSM);
        let operating_fsm = define_fsm!(OperatingFSM);
        let propulsion_fsm = define_fsm!(PropulsionFSM);
        let levitation_fsm = define_fsm!(LevitationFSM);

        Self {
            spawner,
            state: SystemCheck,
            priority_event_pub_sub: Arc::new(PriorityEventPubSub::new(
                event_channel.publisher().unwrap(),
                event_channel.subscriber().unwrap(),
                emergency_channel.publisher().unwrap(),
                emergency_channel.subscriber().unwrap(),
            )),
            high_voltage_fsm,
            emergency_fsm,
            operating_fsm,
            propulsion_fsm,
            levitation_fsm,
        }
    }

    fn handle(&mut self, event: Event) {
        match (&self.state, event) {
            (_, Event::Emergency) => {
                // TODO: attempt shut down
                // TODO: transition to quit
            }
            (SystemCheck, Event::SystemCheckSuccess) => self.transition(Idle),
            (Idle, Event::Activate) => self.transition(Active),
            (Idle, Event::Charge) => self.transition(Charging),
            (Charging, Event::StopCharge) => self.transition(Idle),
            (Active, Event::Operate) => {
                self.propulsion_fsm.run();
                self.levitation_fsm.run();
                self.high_voltage_fsm.run();
                self.operating_fsm.run();
                self.emergency_fsm.run();

                self.transition(Operating);
            },
            (Operating, Event::Quit) => {
                // TODO: add checks for propulsion, levitation
                // if *self.high_voltage_fsm.unwrap().get_state() == HVStates::HighVoltageOn {
                    // TODO: Add event to fsm to stop high voltage and wait to stop
                // }
            }
            _ => {
                // TODO: Problem?
            }
        }
    }
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
//         // TODO: Add event to queue
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