use crate::commons::{Event, EventChannel, PublisherChannel, Runner, SubscriberChannel, Transition};

pub(super) enum PropulsionStates {
    PropulsionOff = 0,
    PropulsionOn,
    PropulsionRunning,
}

pub(super) struct PropulsionFSM {
    state: PropulsionStates,
    // peripherals: // TODO
    pub_channel: PublisherChannel,
    sub_channel: SubscriberChannel,
}

impl PropulsionFSM {
    pub fn new(
        pub_channel: PublisherChannel,
        sub_channel: SubscriberChannel,
        // peripherals // TODO
    ) -> Self {
        Self {
            pub_channel,
            sub_channel,
            state: PropulsionStates::PropulsionOff,
            // peripherals:
        }
    }

    pub fn handle(&mut self, event: Event) {
        match (&self.state, event) {
            (_, Event::Emergency) => {
                // TODO: Send command to stop propulsion if running and to turn off after
                self.transition(PropulsionStates::PropulsionOff);
            }
            (PropulsionStates::PropulsionOff, Event::PropulsionOn) => self.transition(PropulsionStates::PropulsionOn),
            (PropulsionStates::PropulsionOn, Event::PropulsionOff) => self.transition(PropulsionStates::PropulsionOff),
            (PropulsionStates::PropulsionOn, Event::PropulsionRunning) => self.transition(PropulsionStates::PropulsionRunning),
            (PropulsionStates::PropulsionRunning, Event::PropulsionOn) => self.transition(PropulsionStates::PropulsionOn),
            _ => {}
        }
    }
}

impl Runner for PropulsionFSM {
    fn get_sub_channel(&self) -> EventChannel {
        *self.event_queue
    }
}

impl Transition<PropulsionStates> for PropulsionFSM {
    fn entry_method(&self) -> fn() {
        ENTRY_FUNCTION_MAP[&self.state]
    }

    fn exit_method(&self) -> fn() {
        EXIT_FUNCTION_MAP[&self.state]
    }

    fn set_state(&mut self, new_state: PropulsionStates) {
        self.state = new_state;
    }
}

static ENTRY_FUNCTION_MAP: [fn(); 3] = [
    enter_propulsion_off,
    enter_propulsion_on,
    enter_propulsion_running,
];

static EXIT_FUNCTION_MAP: [fn(); 3] = [
    || (),
    || (),
    || (),
];

fn enter_propulsion_on() {
    // TODO: Send command to turn on propulsion
}

fn enter_propulsion_off() {
    // TODO: Send command to turn propulsion off
}

fn enter_propulsion_running() {
    // TODO: Send velocity profile to propulsion
}
