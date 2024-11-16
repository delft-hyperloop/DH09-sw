use crate::commons::{Event, EventChannel, PublisherChannel, Runner, SubscriberChannel, Transition};

pub(super) enum LevitationStates {
    LevitationOff = 0,
    LevitationOn,
}

pub(super) struct LevitationFSM {
    state: LevitationStates,
    // peripherals: // TODO
    pub_channel: PublisherChannel,
    sub_channel: SubscriberChannel,
}

impl LevitationFSM {
    pub fn new(
        pub_channel: PublisherChannel,
        sub_channel: SubscriberChannel,
        // peripherals:
    ) -> Self {
        Self {
            pub_channel,
            sub_channel,
            state: LevitationStates::LevitationOff,
            // peripherals
        }
    }

    pub fn handle(&mut self, event: Event) {
        match (&self.state, event) {
            (LevitationStates::LevitationOn, Event::Emergency) => self.transition(LevitationStates::LevitationOff),
            (LevitationStates::LevitationOn, Event::LevitationOff) => self.transition(LevitationStates::LevitationOff),
            (LevitationStates::LevitationOff, Event::LevitationOn) => self.transition(LevitationStates::LevitationOn),
            _ => {}
        }
    }
}

impl Runner for LevitationFSM {
    fn get_sub_channel(&self) -> EventChannel {
        *self.event_queue
    }
}

impl Transition<LevitationStates> for LevitationFSM {
    fn entry_method(&self) -> fn() {
        ENTRY_FUNCTION_MAP[&self.state]
    }

    fn exit_method(&self) -> fn() {
        EXIT_FUNCTION_MAP[&self.state]
    }

    fn set_state(&mut self, new_state: LevitationStates) {
        self.state = new_state;
    }
}

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


