use core::sync::atomic::Ordering;
use defmt::Format;
use crate::commons::data::Event;
use crate::commons::data::PriorityEventPubSub;
use crate::commons::traits::Runner;
use crate::commons::traits::Transition;
use crate::impl_runner_get_sub_channel;
use crate::impl_transition;
use crate::main_fsm::{LEVITATION_STATE, PROPULSION_STATE, HIGH_VOLTAGE_STATE};

/// Enum representing the different states that the `HighVoltageFSM` will be in.
#[derive(PartialEq, Debug, Clone, Copy, Format)]
pub(super) enum HVStates {
    HighVoltageOff,
    HighVoltageOn,
}

#[derive(Debug)]
pub struct HighVoltageFSM {
    state: HVStates,
    priority_event_pub_sub: PriorityEventPubSub,
}

impl HighVoltageFSM {
    pub fn new(
        priority_event_pub_sub: PriorityEventPubSub,
    ) -> Self {
        Self {
            state: HVStates::HighVoltageOff,
            priority_event_pub_sub,
        }
    }

    /// Handles the events published to the event channel or the emergency
    /// channel
    ///
    /// This method transitions the `HighVoltageFSM` from one state to another
    /// depending on which state it currently is in and what event it
    /// received. If it receives an event that it wasn't expecting in the
    /// current state or if it's meant for one of the other sub-FSMs, it
    /// ignores it.
    ///
    /// # Parameters:
    /// - `event`: Event that can cause a transition in the FSM.
    ///
    /// # Returns:
    /// - `false`: If the FSM receives a `StopSubFSMs` event
    /// - `true`: Otherwise
    async fn handle(&mut self, event: Event) -> bool {
        match (&self.state, event) {
            (_, Event::Emergency) => {
                loop {
                    if !LEVITATION_STATE.load(Ordering::Relaxed)
                        && !PROPULSION_STATE.load(Ordering::Relaxed)
                    {
                        break;
                    }
                }
                // TODO: Send command to turn off
                HIGH_VOLTAGE_STATE.store(false, Ordering::Relaxed);
            }
            (HVStates::HighVoltageOff, Event::StopSubFSMs) => return false,
            (HVStates::HighVoltageOn, Event::HighVoltageOff) =>
            {
                if !LEVITATION_STATE.load(Ordering::Relaxed)
                    && !PROPULSION_STATE.load(Ordering::Relaxed) {
                    self.transition(HVStates::HighVoltageOff, Some(&HIGH_VOLTAGE_STATE)).await
                }
            }
            (HVStates::HighVoltageOff, Event::HighVoltageOn) => {
                self.transition(HVStates::HighVoltageOn, Some(&HIGH_VOLTAGE_STATE)).await
            }
            _ => {}
        }
        true
    }

    /// Method used for testing the FSM
    #[cfg(test)]
    pub fn get_state(&self) -> &HVStates {
        &self.state
    }
}

impl_runner_get_sub_channel!(HighVoltageFSM);
impl_transition!(HighVoltageFSM, HVStates,
    GetState: get_state,
    SetState: set_state,

    OnEntry:
    HighVoltageOff => enter_high_voltage_off,
    HighVoltageOn => enter_high_voltage_on,
);

async fn get_state(fsm: &HighVoltageFSM) -> HVStates {
    fsm.state
}

async fn set_state(fsm: &mut HighVoltageFSM, state: HVStates) {
    fsm.state = state;
}

async fn enter_high_voltage_on(hvfsm: &mut HighVoltageFSM) {
    // TODO: Send CAN command to turn on high voltage

    hvfsm.priority_event_pub_sub.add_event(&Event::HighVoltageOnCanRelay).await;
}

async fn enter_high_voltage_off(hvfsm: &mut HighVoltageFSM) {
    // TODO: Send CAN command to turn off high voltage
}

// #[cfg(test)]
// fn setup_log() {
//     rtt_target::rtt_init_defmt!();
// }
//
// #[cfg(test)]
// #[embedded_test::tests(setup=crate::high_voltage_fsm::setup_log())]
// mod high_voltage_fsm_tests {
//     extern crate embassy_stm32;
//     extern crate embassy_executor;
//
//     use core::sync::atomic::Ordering;
//     use embassy_executor::Spawner;
//     use embassy_sync::blocking_mutex::raw::NoopRawMutex;
//     use crate::commons::{EmergencyChannel, EventChannel};
//     use crate::commons::data::{Event, PriorityEventPubSub};
//     use crate::high_voltage_fsm::{HVStates, HighVoltageFSM};
//     use crate::commons::traits::Runner;
//     use crate::HIGH_VOLTAGE_STATE;
//
//     use embassy_sync::mutex::Mutex;
//     use embassy_sync::pubsub::PubSubChannel;
//     use static_cell::StaticCell;
//
//     static EVENT_CHANNEL: StaticCell<EventChannel> = StaticCell::new();
//     static EMERGENCY_CHANNEL: StaticCell<EmergencyChannel> = StaticCell::new();
//     static FSM: StaticCell<Mutex<NoopRawMutex, HighVoltageFSM>> = StaticCell::new();
//
//     #[test]
//     async fn basic_test() {
//         #[embassy_executor::task]
//         async fn run_fsm(fsm: &'static Mutex<NoopRawMutex, HighVoltageFSM>) {
//             fsm.lock().await.run().await;
//         }
//         // async fn run_fsm(fsm: &'static mut HighVoltageFSM) {
//         //     fsm.run().await;
//         // }
//
//         let event_channel = EVENT_CHANNEL.init(EventChannel::new());
//         let emergency_channel = EMERGENCY_CHANNEL.init(EmergencyChannel::new());
//         let mut fsm = FSM.init(Mutex::new(
//             HighVoltageFSM::new(
//                 PriorityEventPubSub::new(
//                     event_channel.publisher().unwrap(),
//                     event_channel.subscriber().unwrap(),
//                     emergency_channel.publisher().unwrap(),
//                     emergency_channel.subscriber().unwrap(),
//                 ),
//             ))
//         );
//         let publisher = event_channel.publisher().unwrap();
//
//         let spawner = Spawner::for_current_executor().await;
//         // Spawner::spawn(&spawner, run_fsm(&mut fsm)).unwrap();
//
//         let events: [(Event, bool); 4] = [
//             (Event::NoEvent, true),
//             (Event::HighVoltageOff, false),
//             (Event::NoEvent, false),
//             (Event::HighVoltageOn, true),
//         ];
//
//         for (event, running) in events {
//             publisher.publish(event).await;
//             // assert_eq!(state, *FSM.lock().await.get_state());
//             assert_eq!(running, HIGH_VOLTAGE_STATE.load(Ordering::Relaxed));
//             defmt::println!(">>>>>>>>>>>>>>>> Checked state: {:?}", running);
//         }
//
//         publisher.publish(Event::StopSubFSMs).await;
//     }
// }
