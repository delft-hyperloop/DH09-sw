use alloc::sync::Arc;
use crate::commons::{Event, PriorityEventPubSub, Runner, Transition};
use crate::{impl_runner_get_sub_channel, impl_transition};

#[derive(Clone, PartialEq, Debug, Copy)]
pub(super) enum EmergencyStates {
    NotAnEmergency = 0,
    Emergency,
    EmergencyStop,
    EmergencyShutDown,
}

pub(super) struct EmergencyFSM {
    state: EmergencyStates,
    // peripherals: // TODO
    priority_event_pub_sub: Arc<PriorityEventPubSub>,
}

impl EmergencyFSM {
    pub fn new(
        priority_event_pub_sub: PriorityEventPubSub,
    ) -> Self {
        Self {
            state: EmergencyStates::NotAnEmergency,
            priority_event_pub_sub: Arc::new(priority_event_pub_sub),
        }
    }

    fn handle(&mut self, event: Event) {
        match (&self.state, event) {
            (EmergencyStates::NotAnEmergency, Event::Emergency) => {
                self.transition(EmergencyStates::Emergency);
                loop {
                    // if speed == 0 { // TODO
                        self.transition(EmergencyStates::EmergencyStop);
                        break;
                    // }
                }
                // TODO: wait for prop, levi and hv to be off
                self.transition(EmergencyStates::EmergencyShutDown);
            },
            _ => {}
        }
    }
}

impl_runner_get_sub_channel!(EmergencyFSM);
impl_transition!(EmergencyFSM, EmergencyStates);

static ENTRY_FUNCTION_MAP: [fn(); 4] = [
    || (),
    enter_emergency,
    || (),
    || (),
];

static EXIT_FUNCTION_MAP: [fn(); 4] = [
    || (),
    || (),
    || (),
    || (),
];

fn enter_emergency() {
    // TODO
}
