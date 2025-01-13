use core::sync::atomic::Ordering;

use crate::commons::data::Event;
use crate::commons::data::PriorityEventPubSub;
use crate::commons::traits::Runner;
use crate::commons::traits::Transition;
use crate::impl_runner_get_sub_channel;
use crate::impl_transition;
use crate::main_fsm::{LEVITATION_STATE, PROPULSION_STATE};

/// Enum representing the different states that the `LevitationFSM` will be in.
#[derive(Clone, PartialEq, Debug, Copy)]
pub(super) enum LevitationStates {
    LevitationOff,
    LevitationOn,
}

pub(super) struct LevitationFSM {
    state: LevitationStates,
    priority_event_pub_sub: PriorityEventPubSub,
}

impl LevitationFSM {
    pub fn new(
        priority_event_pub_sub: PriorityEventPubSub,
    ) -> Self {
        Self {
            priority_event_pub_sub,
            state: LevitationStates::LevitationOff,
            // peripherals
        }
    }

    /// Handles the events published to the event channel or the emergency
    /// channel
    ///
    /// This method transitions the `LevitationFSM` from one state to another
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
            (LevitationStates::LevitationOn, Event::Emergency) => {
                // TODO
            }
            (LevitationStates::LevitationOff, Event::StopSubFSMs) => return false,
            (LevitationStates::LevitationOn, Event::LevitationOff)
                if !PROPULSION_STATE.load(Ordering::Relaxed) =>
            {
                self.transition(LevitationStates::LevitationOff, Some(&LEVITATION_STATE)).await
            }
            (LevitationStates::LevitationOff, Event::LevitationOn) => {
                self.transition(LevitationStates::LevitationOn, Some(&LEVITATION_STATE)).await
            }
            _ => {}
        }
        true
    }
}

impl_runner_get_sub_channel!(LevitationFSM);
impl_transition!(LevitationFSM, LevitationStates);

/// Maps an index to a function that should be called upon entering a new state.
///
/// The indexes correspond to the index of each state in `LevitationStates`.
const ENTRY_FUNCTION_MAP: [fn(&mut LevitationFSM); 2] = [enter_levitation_off, enter_levitation_on];

/// Maps an index to a function that should be called upon exiting a state.
///
/// The indexes correspond to the index of each state in `LevitationStates`.
const EXIT_FUNCTION_MAP: [fn(&mut LevitationFSM); 2] = [|levi_fsm| (), |levi_fsm| ()];

fn enter_levitation_off(levi_fsm: &mut LevitationFSM) {
    // TODO
}

fn enter_levitation_on(levi_fsm: &mut LevitationFSM) {
    // TODO
}
