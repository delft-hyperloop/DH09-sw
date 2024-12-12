#![no_std]

use crate::commons::data::Event;
use crate::commons::data::PriorityEventPubSub;
use crate::commons::traits::Runner;
use crate::commons::traits::Transition;
use crate::impl_runner_get_sub_channel;
use crate::impl_transition;
use crate::EMERGENCY_STATE;

/// Enum representing the different states that the `EmergencyFSM` will be in.
#[derive(Clone, PartialEq, Debug, Copy)]
pub(super) enum EmergencyStates {
    NotAnEmergency,
    Emergency,
    EmergencyStop,
    EmergencyShutDown,
}

pub(super) struct EmergencyFSM {
    state: EmergencyStates,
    // peripherals: // TODO
    priority_event_pub_sub: PriorityEventPubSub,
}

impl EmergencyFSM {
    pub fn new(priority_event_pub_sub: PriorityEventPubSub) -> Self {
        Self {
            state: EmergencyStates::NotAnEmergency,
            priority_event_pub_sub: priority_event_pub_sub,
        }
    }

    /// Handles the events published to the event channel or the emergency
    /// channel
    ///
    /// This method transitions the `EmergencyFSM` from one state to another
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
            (EmergencyStates::NotAnEmergency, Event::Emergency) => {
                self.transition(EmergencyStates::Emergency, Some(&EMERGENCY_STATE));
                // TODO: Activate EBS?
                loop {
                    // if speed == 0 { // TODO
                    self.transition(EmergencyStates::EmergencyStop, None);
                    // TODO: Send CAN command to stop levitation
                    break;
                    // }
                }

                // TODO: waiting on some answers from tech integration for this
                // shut_down().await;

                self.transition(EmergencyStates::EmergencyShutDown, None);
            }
            (EmergencyStates::NotAnEmergency, Event::StopSubFSMs) => return false,
            _ => {}
        }
        true
    }
}

impl_runner_get_sub_channel!(EmergencyFSM);
impl_transition!(EmergencyFSM, EmergencyStates);

/// Maps an index to a function that should be called upon entering a new state.
///
/// The indexes correspond to the index of each state in `EmergencyStates`.
const ENTRY_FUNCTION_MAP: [fn(); 4] = [|| (), || (), || (), || ()];

/// Maps an index to a function that should be called upon exiting a state.
///
/// The indexes correspond to the index of each state in `MainStates`.
const EXIT_FUNCTION_MAP: [fn(); 4] = [|| (), || (), || (), || ()];
