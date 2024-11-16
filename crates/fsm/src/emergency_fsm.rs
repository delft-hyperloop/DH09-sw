use crate::commons::{Event, EventChannel, PublisherChannel, Runner, SubscriberChannel, Transition};

pub(super) enum EmergencyStates {
    NotAnEmergency = 0,
    Emergency,
    EmergencyStop,
    EmergencyShutDown,
}

pub(super) struct EmergencyFSM {
    state: EmergencyStates,
    // peripherals: // TODO
    pub_channel: PublisherChannel,
    sub_channel: SubscriberChannel,
}

impl EmergencyFSM {
    pub fn new(
        pub_channel: PublisherChannel,
        sub_channel: SubscriberChannel,
    ) -> Self {
        Self {
            state: EmergencyStates::NotAnEmergency,
            pub_channel,
            sub_channel,
        }
    }

    pub fn handle(&mut self, event: Event) {
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

impl Runner for EmergencyFSM {
    fn get_sub_channel(&self) -> EventChannel {
        *self.event_queue
    }
}

impl Transition<EmergencyStates> for EmergencyFSM {
    fn entry_method(&self) -> fn() {
        ENTRY_FUNCTION_MAP[&self.state]
    }

    fn exit_method(&self) -> fn() {
        EXIT_FUNCTION_MAP[&self.state]
    }

    fn set_state(&mut self, new_state: EmergencyStates) {
        self.state = new_state;
    }
}

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
