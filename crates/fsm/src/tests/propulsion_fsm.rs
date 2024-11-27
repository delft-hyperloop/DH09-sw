use static_cell::StaticCell;
use crate::commons::{EmergencyChannel, EventChannel};
use crate::commons::data::{Event, PriorityEventPubSub};
use crate::commons::traits::Runner;
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
