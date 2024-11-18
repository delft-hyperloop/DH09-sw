use crate::commons::{Event, PublisherChannel, Runner, SubscriberChannel, Transition};
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
    pub_channel: PublisherChannel,
    sub_channel: SubscriberChannel,
    // peripherals: // TODO
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
            (OperatingStates::Demo, Event::Accelerate {velocity_profile: _velocity_profile}) => self.transition(OperatingStates::Accelerating),
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
