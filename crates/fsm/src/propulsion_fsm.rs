#![no_std]
use crate::commons::data::Event;
use crate::commons::data::PriorityEventPubSub;
use crate::commons::traits::Runner;
use crate::commons::traits::Transition;
use crate::impl_runner_get_sub_channel;
use crate::impl_transition;
use crate::PROPULSION_STATE;

/// Enum representing the different states that the `PropulsionFSM` will be in.
#[derive(PartialEq, Debug, Clone, Copy)]
pub(super) enum PropulsionStates {
    PropulsionOff,
    PropulsionOn,
    PropulsionRunning,
}

pub(super) struct PropulsionFSM {
    state: PropulsionStates,
    priority_event_pub_sub: PriorityEventPubSub,
    _velocity_profile: u8, /* TODO: Change to actual velocity profile
                            * peripherals: // TODO */
}

impl PropulsionFSM {
    pub fn new(
        priority_event_pub_sub: PriorityEventPubSub,
        // peripherals // TODO
    ) -> Self {
        Self {
            priority_event_pub_sub: priority_event_pub_sub,
            state: PropulsionStates::PropulsionOff,
            _velocity_profile: 0, /* TODO: Change to actual velocity profile
                                   * peripherals: // TODO */
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_state(&self) -> &PropulsionStates {
        &self.state
    }

    /// Handles the events published to the event channel or the emergency
    /// channel
    ///
    /// This method transitions the `PropulsionFSM` from one state to another
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
                if self.state == PropulsionStates::PropulsionRunning {
                    // TODO: Send command to stop running and turn off
                } else if self.state == PropulsionStates::PropulsionOn {
                    // TODO: Send command to turn off propulsion completely
                }
                self.transition(PropulsionStates::PropulsionOff, Some(&PROPULSION_STATE));
            }
            (PropulsionStates::PropulsionOff, Event::StopSubFSMs) => return false,
            (PropulsionStates::PropulsionOff, Event::PropulsionOn) => {
                self.transition(PropulsionStates::PropulsionOn, Some(&PROPULSION_STATE))
            }
            (PropulsionStates::PropulsionOn, Event::PropulsionOff) => {
                self.transition(PropulsionStates::PropulsionOff, Some(&PROPULSION_STATE))
            }
            (
                PropulsionStates::PropulsionOn,
                Event::Accelerate {
                    velocity_profile: _velocity_profile,
                },
            ) => {
                // TODO: Send self.velocity_profile to propulsion
                self.transition(PropulsionStates::PropulsionRunning, None)
            }
            (PropulsionStates::PropulsionRunning, Event::Brake) => {
                self.transition(PropulsionStates::PropulsionOn, None)
            }
            _ => {}
        }
        true
    }
}

impl_runner_get_sub_channel!(PropulsionFSM);
impl_transition!(PropulsionFSM, PropulsionStates);

/// Maps an index to a function that should be called upon entering a new state.
///
/// The indexes correspond to the index of each state in `PropulsionStates`.
const ENTRY_FUNCTION_MAP: [fn(); 3] = [enter_propulsion_off, enter_propulsion_on, || ()];

/// Maps an index to a function that should be called upon exiting a state.
///
/// The indexes correspond to the index of each state in `PropulsionStates`.
const EXIT_FUNCTION_MAP: [fn(); 3] = [|| (), || (), || ()];

fn enter_propulsion_on() {
    // TODO: Send command to turn on propulsion
}

fn enter_propulsion_off() {
    // TODO: Send command to turn propulsion off
}