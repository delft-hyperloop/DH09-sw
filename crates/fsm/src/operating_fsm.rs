use alloc::sync::Arc;
use crate::commons::{Event, PriorityEventPubSub, Runner, Transition};
use crate::{impl_runner_get_sub_channel, impl_transition};

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

static ENTRY_FUNCTION_MAP: [fn(); 5] = [
    || (), // Demo
    || (), // Accelerating
    enter_braking,
    || (), // Cruising
    || (), // Shut Down
];

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
