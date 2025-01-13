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

/// Enum representing the different states that the `OperatingFSM` will be in.
#[derive(Clone, PartialEq, Debug, Copy)]
pub(super) enum OperatingStates {
    Demo,
    Accelerating,
    Braking,
    Cruising,
}

pub(super) struct OperatingFSM {
    state: OperatingStates,
    priority_event_pub_sub: PriorityEventPubSub,
    // peripherals: // TODO
}

impl OperatingFSM {
    pub fn new(priority_event_pub_sub: PriorityEventPubSub) -> Self {
        Self {
            state: OperatingStates::Demo,
            priority_event_pub_sub,
        }
    }

    /// Handles the events published to the event channel or the emergency
    /// channel
    ///
    /// This method transitions the `OperatingFSM` from one state to another
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
        // Didn't include emergency event here. It shouldn't be handled here and it will
        // be useful to see the state in which this was when the emergency
        // happened.
        match (&self.state, event) {
            (
                OperatingStates::Demo,
                Event::Accelerate {
                    velocity_profile: _velocity_profile,
                },
            ) => {
                if HIGH_VOLTAGE_STATE.load(Ordering::Relaxed)
                    && LEVITATION_STATE.load(Ordering::Relaxed)
                    && PROPULSION_STATE.load(Ordering::Relaxed)
                {
                    self.transition(OperatingStates::Accelerating, None);
                }
            }
            (OperatingStates::Demo, Event::ShutDown) => return false,
            (OperatingStates::Accelerating, Event::Cruise) => {
                self.transition(OperatingStates::Cruising, None)
            }
            (OperatingStates::Accelerating, Event::Brake) => {
                self.transition(OperatingStates::Braking, None)
            }
            (OperatingStates::Cruising, Event::Brake) => {
                self.transition(OperatingStates::Braking, None)
            }
            (OperatingStates::Braking, Event::Demo) => {
                loop {
                    // if speed == 0 { // TODO
                    self.transition(OperatingStates::Demo, None);
                    break;
                    // }
                }
            }
            _ => {}
        }
        true
    }
}

impl_runner_get_sub_channel!(OperatingFSM);
impl_transition!(OperatingFSM, OperatingStates);

/// Maps an index to a function that should be called upon entering a new state.
///
/// The indexes correspond to the index of each state in `OperatingStates`.
const ENTRY_FUNCTION_MAP: [fn(&mut OperatingFSM); 5] = [
    |op_fsm| (), // Demo
    |op_fsm| (), // Accelerating
    enter_braking,
    |op_fsm| (), // Cruising
    |op_fsm| (), // Shut Down
];

/// Maps an index to a function that should be called upon exiting a state.
///
/// The indexes correspond to the index of each state in `OperatingStates`.
const EXIT_FUNCTION_MAP: [fn(&mut OperatingFSM); 5] = [
    |op_fsm| (), // Demo
    |op_fsm| (), // Accelerating
    |op_fsm| (), // Braking
    |op_fsm| (), // Cruising
    |op_fsm| (), // Shut Down
];

fn enter_braking(op_fsm: &mut OperatingFSM) {
    // TODO: Send braking command to braking PCB
}
