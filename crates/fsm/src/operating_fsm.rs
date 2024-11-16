use crate::commons::{Event, EventChannel, PublisherChannel, Runner, SubscriberChannel, Transition};

pub(super) enum OperatingStates {
    Demo = 0,
    Accelerating,
    Braking,
    Cruising,
    ShutDown
}

pub(super) struct OperatingFSM {
    state: OperatingStates,
    pub_channel: PublisherChannel,
    sub_channel: SubscriberChannel,
    // peripherals:
}

impl OperatingFSM {
    pub fn new(
        pub_channel: PublisherChannel,
        sub_channel: SubscriberChannel,
    ) -> Self {
        Self {
            state: OperatingStates::Demo,
            pub_channel,
            sub_channel,
        }
    }

    pub fn handle(&mut self, event: Event) {
        // TODO: Don't forget to check if hv is on, levi is on etc before going to accelerate
        match (&self.state, event) {
            (_, Event::Emergency) => {
                // TODO: Decide if we change state and if anything happens here
            }
            (OperatingStates::Demo, Event::Accelerate) => self.transition(OperatingStates::Accelerating),
            (OperatingStates::Demo, Event::ShutDown) => self.transition(OperatingStates::ShutDown),
            (OperatingStates::Accelerating, Event::Cruise) => self.transition(OperatingStates::Cruising),
            (OperatingStates::Accelerating, Event::Brake) => self.transition(OperatingStates::Braking),
            (OperatingStates::Cruising, Event::Brake) => self.transition(OperatingStates::Braking),
            (OperatingStates::Braking, Event::Demo) => {
                loop {
                    // if speed == 0 { // TODO
                        self.transition(OperatingStates::Demo);
                    // }
                }
            }
            _ => {}
        }
    }
}

impl Runner for OperatingFSM {
    fn get_sub_channel(&self) -> EventChannel {
        *self.event_queue
    }
}

impl Transition<OperatingStates> for OperatingFSM {
    fn entry_method(&self) -> fn() {
        ENTRY_FUNCTION_MAP[&self.state]
    }

    fn exit_method(&self) -> fn() {
        EXIT_FUNCTION_MAP[&self.state]
    }

    fn set_state(&mut self, new_state: OperatingStates) {
        self.state = new_state;
    }
}

static ENTRY_FUNCTION_MAP: [fn(); 5] = [
    || (),
    || (),
    enter_braking,
    || (),
    || (),
];

static EXIT_FUNCTION_MAP: [fn(); 5] = [
    || (),
    || (),
    || (),
    || (),
    || (),
];

fn enter_braking() {
    // TODO: Send braking command to braking PCB, check if prop is running
}
