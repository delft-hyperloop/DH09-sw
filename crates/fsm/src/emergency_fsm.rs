//! This module contains the struct used for the Emergency FSM.

use crate::commons::data::{Event, PriorityEventPubSub};
use crate::commons::traits::Runner;
use crate::commons::traits::Transition;
use crate::impl_runner_get_sub_channel;
use crate::impl_transition;
use crate::main_fsm::EMERGENCY_STATE;

/// Enum representing the different states that the `EmergencyFSM` will be in.
#[derive(Clone, PartialEq, Debug, Copy, defmt::Format)]
pub(super) enum EmergencyStates {
    NotAnEmergency,
    Emergency,
    EmergencyStop,
    EmergencyShutDown,
}

#[derive(Debug)]
pub(super) struct EmergencyFSM {
    state: EmergencyStates,
    priority_event_pub_sub: PriorityEventPubSub,
}

impl EmergencyFSM {
    pub fn new(priority_event_pub_sub: PriorityEventPubSub) -> Self {
        Self {
            state: EmergencyStates::NotAnEmergency,
            priority_event_pub_sub,
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
                self.transition(EmergencyStates::Emergency, Some(&EMERGENCY_STATE)).await;
                // TODO: Activate EBS?
                loop {
                    // if speed == 0 { // TODO
                    self.transition(EmergencyStates::EmergencyStop, None).await;
                    // TODO: Send CAN command to stop levitation
                    break;
                    // }
                }

                // TODO: waiting on some answers from tech integration for this
                // shut_down().await;

                self.transition(EmergencyStates::EmergencyShutDown, None).await;
            }
            (EmergencyStates::NotAnEmergency, Event::StopSubFSMs) => return false,
            _ => {}
        }
        true
    }

    #[allow(dead_code)]
    pub fn get_state(&self) -> &EmergencyStates {
        &self.state
    }
}

impl_runner_get_sub_channel!(EmergencyFSM);
impl_transition!(EmergencyFSM, EmergencyStates,
    GetState: get_state,
    SetState: set_state,
);

async fn get_state(fsm: &EmergencyFSM) -> EmergencyStates {
    *fsm.get_state()
}

async fn set_state(fsm: &mut EmergencyFSM, state: EmergencyStates) {
    fsm.state = state;
}

