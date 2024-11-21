use alloc::sync::Arc;
use core::sync::atomic::Ordering;
use crate::commons::{Event, PriorityEventPubSub, Runner, Transition};
use crate::{impl_runner_get_sub_channel, impl_transition, EMERGENCY_STATE, HIGH_VOLTAGE_STATE, LEVITATION_STATE, PROPULSION_STATE};

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

    async fn handle(&mut self, event: Event) -> bool {
        match (&self.state, event) {
            (EmergencyStates::NotAnEmergency, Event::Emergency) => {
                self.transition(EmergencyStates::Emergency, Some(&EMERGENCY_STATE));
                loop {
                    // if speed == 0 { // TODO
                        self.transition(EmergencyStates::EmergencyStop, None);
                        break;
                    // }
                }
                // TODO: wait for prop, levi and hv to be off
                loop {
                    if !PROPULSION_STATE.load(Ordering::Relaxed) && !LEVITATION_STATE.load(Ordering::Relaxed)
                        && !HIGH_VOLTAGE_STATE.load(Ordering::Relaxed) {
                        break;
                    }
                }
                self.transition(EmergencyStates::EmergencyShutDown, None);
            },
            (EmergencyStates::NotAnEmergency, Event::StopSubFSMs) => return false,
            _ => {}
        }
        true
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
