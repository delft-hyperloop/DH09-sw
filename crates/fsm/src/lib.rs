#![no_std]
#![no_main]

pub mod commons;
mod high_voltage_fsm;
mod emergency_fsm;
mod operating_fsm;
mod propulsion_fsm;
mod levitation_fsm;

use core::cmp::PartialEq;
use embassy_executor::Spawner;
use commons::Event;
use MainStates::*;
use crate::commons::{EventChannel, PublisherChannel, Runner, SubscriberChannel, Transition};
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
    pub_channel: PublisherChannel,
    sub_channel: SubscriberChannel,
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
        event_channel: &EventChannel,
    ) -> Self {
        let pub_channel = event_channel.publisher().unwrap();
        let sub_channel= event_channel.subscriber().unwrap();

        let high_voltage_fsm = HighVoltageFSM::new(event_channel.publisher().unwrap(), event_channel.subscriber().unwrap());
        let emergency_fsm = EmergencyFSM::new(event_channel.publisher().unwrap(), event_channel.subscriber().unwrap());
        let operating_fsm = OperatingFSM::new(event_channel.publisher().unwrap(), event_channel.subscriber().unwrap());
        let propulsion_fsm = PropulsionFSM::new(event_channel.publisher().unwrap(), event_channel.subscriber().unwrap());
        let levitation_fsm = LevitationFSM::new(event_channel.publisher().unwrap(), event_channel.subscriber().unwrap());

        Self {
            spawner,
            state: SystemCheck,
            pub_channel,
            sub_channel,
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