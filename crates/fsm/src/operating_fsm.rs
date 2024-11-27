use alloc::sync::Arc;
use crate::{impl_runner_get_sub_channel, impl_transition};
use crate::commons::data::{Event, PriorityEventPubSub};
use crate::commons::traits::{Transition, Runner};

/// Enum representing the different states that the `OperatingFSM` will be in.
#[derive(Clone, PartialEq, Debug, Copy)]
pub(super) enum OperatingStates {
    Demo = 0,
    Accelerating,
    Braking,
    Cruising,
    ShutDown
}

pub(super) struct OperatingFSM {
    state: OperatingStates,
    priority_event_pub_sub: Arc<PriorityEventPubSub>,
    // peripherals: // TODO
}

impl OperatingFSM {
    pub fn new(
        priority_event_pub_sub: PriorityEventPubSub,
    ) -> Self {
        Self {
            state: OperatingStates::Demo,
            priority_event_pub_sub: Arc::new(priority_event_pub_sub),
        }
    }

    /// Handles the events published to the event channel or the emergency channel
    ///
    /// This method transitions the `OperatingFSM` from one state to another depending on
    /// which state it currently is in and what event it received. If it receives an
    /// event that it wasn't expecting in the current state or if it's meant for one of the
    /// other sub-FSMs, it ignores it.
    ///
    /// # Parameters:
    /// - `event`: Event that can cause a transition in the FSM.
    ///
    /// # Returns:
    /// - `false`: If the FSM receives a `StopSubFSMs` event
    /// - `true`: Otherwise
    async fn handle(&mut self, event: Event) -> bool {
        // TODO: Don't forget to check if hv is on, levi is on etc before going to accelerate
        match (&self.state, event) {
            (_, Event::Emergency) => {
                // TODO: Decide if we change state and if anything happens here
            }
            (OperatingStates::Demo, Event::Accelerate {velocity_profile: _velocity_profile}) => self.transition(OperatingStates::Accelerating, None),
            (OperatingStates::Demo, Event::ShutDown) => return false,
            (OperatingStates::Accelerating, Event::Cruise) => self.transition(OperatingStates::Cruising, None),
            (OperatingStates::Accelerating, Event::Brake) => self.transition(OperatingStates::Braking, None),
            (OperatingStates::Cruising, Event::Brake) => self.transition(OperatingStates::Braking, None),
            (OperatingStates::Braking, Event::Demo) => {
                loop {
                    // if speed == 0 { // TODO
                        self.transition(OperatingStates::Demo, None);
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
static ENTRY_FUNCTION_MAP: [fn(); 5] = [
    || (), // Demo
    || (), // Accelerating
    enter_braking,
    || (), // Cruising
    || (), // Shut Down
];

/// Maps an index to a function that should be called upon exiting a state.
///
/// The indexes correspond to the index of each state in `OperatingStates`.
static EXIT_FUNCTION_MAP: [fn(); 5] = [
    || (), // Demo
    || (), // Accelerating
    || (), // Braking
    || (), // Cruising
    || (), // Shut Down
];

fn enter_braking() {
    // TODO: Send braking command to braking PCB, check if prop is running
}
