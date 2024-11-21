use alloc::sync::Arc;
use core::sync::atomic::Ordering;
use crate::commons::{Event, PriorityEventPubSub, Runner, Transition};
use crate::{impl_runner_get_sub_channel, impl_transition, LEVITATION_STATE, PROPULSION_STATE};

#[derive(Clone, PartialEq, Debug, Copy)]
pub(super) enum LevitationStates {
    LevitationOff = 0,
    LevitationOn,
}

pub(super) struct LevitationFSM {
    state: LevitationStates,
    // peripherals: // TODO
    priority_event_pub_sub: Arc<PriorityEventPubSub>,
}

impl LevitationFSM {
    pub fn new(
        priority_event_pub_sub: PriorityEventPubSub,
        // peripherals:
    ) -> Self {
        Self {
            priority_event_pub_sub: Arc::new(priority_event_pub_sub),
            state: LevitationStates::LevitationOff,
            // peripherals
        }
    }

    async fn handle(&mut self, event: Event) -> bool {
        match (&self.state, event) {
            (LevitationStates::LevitationOn, Event::Emergency) => {
                // TODO
            },
            (LevitationStates::LevitationOff, Event::StopSubFSMs) => return false,
            (LevitationStates::LevitationOn, Event::LevitationOff) if !PROPULSION_STATE.load(Ordering::Relaxed)
                => self.transition(LevitationStates::LevitationOff, Some(&LEVITATION_STATE)),
            (LevitationStates::LevitationOff, Event::LevitationOn) => self.transition(LevitationStates::LevitationOn, Some(&LEVITATION_STATE)),
            _ => {}
        }
        true
    }
}

impl_runner_get_sub_channel!(LevitationFSM);
impl_transition!(LevitationFSM, LevitationStates);

static ENTRY_FUNCTION_MAP: [fn(); 2] = [
    enter_levitation_off,
    enter_levitation_on,
];

static EXIT_FUNCTION_MAP: [fn(); 2] = [
    || (),
    || (),
];

fn enter_levitation_off() {
    // TODO
}

fn enter_levitation_on() {
    // TODO
}


