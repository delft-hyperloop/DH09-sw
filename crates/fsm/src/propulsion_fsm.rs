use crate::commons::data::Event;
use crate::commons::data::PriorityEventPubSub;
use crate::commons::traits::Runner;
use crate::commons::traits::Transition;
use crate::impl_runner_get_sub_channel;
use crate::impl_transition;
use crate::main_fsm::PROPULSION_STATE;

/// Enum representing the different states that the `PropulsionFSM` will be in.
#[derive(PartialEq, Debug, Clone, Copy, defmt::Format)]
pub(super) enum PropulsionStates {
    PropulsionOff,
    PropulsionOn,
    PropulsionRunning,
}

pub(super) struct PropulsionFSM {
    state: PropulsionStates,
    priority_event_pub_sub: PriorityEventPubSub,
    _velocity_profile: u8, // TODO: Change to actual velocity profile
}

impl PropulsionFSM {
    pub fn new(
        priority_event_pub_sub: PriorityEventPubSub,
    ) -> Self {
        Self {
            priority_event_pub_sub,
            state: PropulsionStates::PropulsionOff,
            _velocity_profile: 0, // TODO: Change to actual velocity profile
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
                self.transition(PropulsionStates::PropulsionOff, Some(&PROPULSION_STATE)).await;
            }
            (PropulsionStates::PropulsionOff, Event::StopSubFSMs) => return false,
            (PropulsionStates::PropulsionOff, Event::PropulsionOn) => {
                self.transition(PropulsionStates::PropulsionOn, Some(&PROPULSION_STATE)).await
            }
            (PropulsionStates::PropulsionOn, Event::PropulsionOff) => {
                self.transition(PropulsionStates::PropulsionOff, Some(&PROPULSION_STATE)).await
            }
            (
                PropulsionStates::PropulsionOn,
                Event::Accelerate {
                    velocity_profile: _velocity_profile,
                },
            ) => {
                // TODO: Send self.velocity_profile to propulsion
                self.transition(PropulsionStates::PropulsionRunning, None).await
            }
            (PropulsionStates::PropulsionRunning, Event::Brake) => {
                self.transition(PropulsionStates::PropulsionOn, None).await
            }
            _ => {}
        }
        true
    }
}

impl_runner_get_sub_channel!(PropulsionFSM);
impl_transition!(PropulsionFSM, PropulsionStates,
    GetState: get_state,
    SetState: set_state,

    OnEntry:
    PropulsionOff => enter_propulsion_off,
    PropulsionOn => enter_propulsion_on,
);

async fn get_state(fsm: &PropulsionFSM) -> PropulsionStates {
    *fsm.get_state()
}

async fn set_state(fsm: &mut PropulsionFSM, state: PropulsionStates) {
    fsm.state = state;
}

async fn enter_propulsion_on(fsm: &mut PropulsionFSM) {
    // TODO: Send command to turn on propulsion
}

async fn enter_propulsion_off(fsm: &mut PropulsionFSM) {
    // TODO: Send command to turn propulsion off
}