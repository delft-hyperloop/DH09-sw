use alloc::sync::Arc;
use crate::commons::{Event, PriorityEventPubSub, Runner, Transition};
use crate::{impl_runner_get_sub_channel, impl_transition, PROPULSION_STATE};

#[derive(PartialEq, Debug, Clone, Copy)]
pub(super) enum PropulsionStates {
    PropulsionOff = 0,
    PropulsionOn,
    PropulsionRunning,
}

pub(super) struct PropulsionFSM {
    state: PropulsionStates,
    priority_event_pub_sub: Arc<PriorityEventPubSub>,
    velocity_profile: u8, // TODO: Change to actual velocity profile
    // peripherals: // TODO
}

impl PropulsionFSM {
    pub fn new(
        priority_event_pub_sub: PriorityEventPubSub,
        // peripherals // TODO
    ) -> Self {
        Self {
            priority_event_pub_sub: Arc::new(priority_event_pub_sub),
            state: PropulsionStates::PropulsionOff,
            velocity_profile: 0, // TODO: Change to actual velocity profile
            // peripherals: // TODO
        }
    }

    #[allow(dead_code)]
    pub fn get_state(&self) -> &PropulsionStates {
        &self.state
    }

    async fn handle(&mut self, event: Event) -> bool {
        match (&self.state, event) {
            (_, Event::Emergency) => {
                if self.state == PropulsionStates::PropulsionRunning {
                    // TODO: Send command to stop running and turn off
                } else if self.state == PropulsionStates::PropulsionOn {
                    // TODO: Send command to turn off propulsion completely
                }
                self.transition(PropulsionStates::PropulsionOff, Some(&PROPULSION_STATE));
            },
            (PropulsionStates::PropulsionOff, Event::StopSubFSMs) => return false,
            (PropulsionStates::PropulsionOff, Event::PropulsionOn) => self.transition(PropulsionStates::PropulsionOn, Some(&PROPULSION_STATE)),
            (PropulsionStates::PropulsionOn, Event::PropulsionOff) => self.transition(PropulsionStates::PropulsionOff, Some(&PROPULSION_STATE)),
            (PropulsionStates::PropulsionOn, Event::PropulsionRunning) => {
                // TODO: Send self.velocity_profile to propulsion
                self.transition(PropulsionStates::PropulsionRunning, None)
            },
            (PropulsionStates::PropulsionRunning, Event::PropulsionOn) => self.transition(PropulsionStates::PropulsionOn, None),
            _ => {}
        }
        true
    }
}

impl_runner_get_sub_channel!(PropulsionFSM);
impl_transition!(PropulsionFSM, PropulsionStates);

static ENTRY_FUNCTION_MAP: [fn(); 3] = [
    enter_propulsion_off,
    enter_propulsion_on,
    || (),
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

#[cfg(test)]
mod tests {
    use static_cell::StaticCell;
    use crate::commons::{EmergencyChannel, Event, EventChannel, PriorityEventPubSub, Runner};
    use crate::propulsion_fsm::{PropulsionFSM, PropulsionStates};

    #[test]
    fn test_basic_transitions() {
        static CHANNEL: StaticCell<EventChannel> = static_cell::StaticCell::new();
        static EMERGENCY_CHANNEL: StaticCell<EmergencyChannel> = static_cell::StaticCell::new();

        let event_channel = CHANNEL.init(EventChannel::new());
        let emergency_channel = EMERGENCY_CHANNEL.init(EmergencyChannel::new());

        let pub_channel = event_channel.publisher().unwrap();
        let pub_emergency_channel = emergency_channel.publisher().unwrap();

        let mut fsm = PropulsionFSM::new(
            PriorityEventPubSub::new(
                event_channel.publisher().unwrap(),
                event_channel.subscriber().unwrap(),
                emergency_channel.publisher().unwrap(),
                emergency_channel.subscriber().unwrap(),
            ),
        );

        // Need a separate task?
        fsm.run();

        // TODO: Also check if the commands have been sent

        pub_channel.publish(Event::PropulsionOn);
        assert_eq!(*fsm.get_state(), PropulsionStates::PropulsionOn);

        pub_channel.publish(Event::PropulsionRunning);
        assert_eq!(*fsm.get_state(), PropulsionStates::PropulsionRunning);

        pub_channel.publish(Event::PropulsionOn);
        assert_eq!(*fsm.get_state(), PropulsionStates::PropulsionOn);

        pub_channel.publish(Event::PropulsionOff);
        assert_eq!(*fsm.get_state(), PropulsionStates::PropulsionOff);
    }

    // #[test]
    // fn test_ignores_other_events() {
    //     // TODO
    // }

    // #[test]
    // fn test_emergency() {
    //     // TODO
    // }

    // #[test]
    // fn test_calls_entry_functions() {
    //     // TODO
    // }

    // #[test]
    // fn test_lags_events() {
    //     // TODO
    // }
}