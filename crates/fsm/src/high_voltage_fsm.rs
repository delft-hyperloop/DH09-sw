#![no_std]
use core::sync::atomic::Ordering;

use crate::commons::data::Event;
use crate::commons::data::PriorityEventPubSub;
use crate::commons::traits::Runner;
use crate::commons::traits::Transition;
use crate::impl_runner_get_sub_channel;
use crate::impl_transition;
use crate::HIGH_VOLTAGE_STATE;
use crate::LEVITATION_STATE;
use crate::PROPULSION_STATE;

/// Enum representing the different states that the `HighVoltageFSM` will be in.
#[derive(PartialEq, Debug, Clone, Copy)]
pub(super) enum HVStates {
    HighVoltageOff,
    HighVoltageOn,
}

pub(super) struct HighVoltageFSM {
    state: HVStates,
    priority_event_pub_sub: PriorityEventPubSub,
    // peripherals: // TODO
}

impl HighVoltageFSM {
    pub fn new(
        priority_event_pub_sub: PriorityEventPubSub,
        //peripherals: // TODO
    ) -> Self {
        Self {
            state: HVStates::HighVoltageOn,
            priority_event_pub_sub: priority_event_pub_sub,
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
            (HVStates::HighVoltageOn, Event::HighVoltageOff)
                if !LEVITATION_STATE.load(Ordering::Relaxed)
                    && !LEVITATION_STATE.load(Ordering::Relaxed) =>
            {
                self.transition(HVStates::HighVoltageOff, Some(&HIGH_VOLTAGE_STATE))
            }
            (HVStates::HighVoltageOff, Event::HighVoltageOn) => {
                self.transition(HVStates::HighVoltageOn, Some(&HIGH_VOLTAGE_STATE))
            }
            _ => {}
        }
        true
    }

    #[allow(dead_code)]
    pub(super) fn get_state(&self) -> &HVStates {
        &self.state
    }
}

impl_runner_get_sub_channel!(HighVoltageFSM);
impl_transition!(HighVoltageFSM, HVStates);

/// Maps an index to a function that should be called upon entering a new state.
///
/// The indexes correspond to the index of each state in `HVStates`.
const ENTRY_FUNCTION_MAP: [fn(); 2] = [enter_high_voltage_off, enter_high_voltage_on];

/// Maps an index to a function that should be called upon exiting a state.
///
/// The indexes correspond to the index of each state in `HVStates`.
const EXIT_FUNCTION_MAP: [fn(); 2] = [|| (), || ()];

fn enter_high_voltage_on() {
    // TODO: Send CAN command to turn on high voltage
}

fn enter_high_voltage_off() {
    // TODO: Send CAN command to turn off high voltage
}
