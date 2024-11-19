use alloc::sync::Arc;
use crate::commons::{Event, PriorityEventPubSub, Runner, Transition};
use crate::{impl_runner_get_sub_channel, impl_transition};

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
            // peripherals:
        }
    }

    pub fn get_state(&self) -> &PropulsionStates {
        &self.state
    }

    fn handle(&mut self, event: Event) {
        match (&self.state, event) {
            (_, Event::Emergency) => {
                // TODO: Send command to stop propulsion if running and to turn off after
                self.transition(PropulsionStates::PropulsionOff);
            }
            (PropulsionStates::PropulsionOff, Event::PropulsionOn) => self.transition(PropulsionStates::PropulsionOn),
            (PropulsionStates::PropulsionOn, Event::PropulsionOff) => self.transition(PropulsionStates::PropulsionOff),
            (PropulsionStates::PropulsionOn, Event::PropulsionRunning) => {
                // TODO: Send self.velocity_profile to propulsion
                self.transition(PropulsionStates::PropulsionRunning)
            },
            (PropulsionStates::PropulsionRunning, Event::PropulsionOn) => self.transition(PropulsionStates::PropulsionOn),
            _ => {}
        }
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

        let _result = pub_channel.publish(Event::PropulsionOn);
        assert_eq!(*fsm.get_state(), PropulsionStates::PropulsionOn);

        let _result = pub_channel.publish(Event::PropulsionRunning);
        assert_eq!(*fsm.get_state(), PropulsionStates::PropulsionRunning);

        let _result = pub_channel.publish(Event::PropulsionOn);
        assert_eq!(*fsm.get_state(), PropulsionStates::PropulsionOn);

        let _result = pub_channel.publish(Event::PropulsionOff);
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