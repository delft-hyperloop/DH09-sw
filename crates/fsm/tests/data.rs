#![no_std]
#![no_main]

#[cfg(test)]
fn setup_log() {
    rtt_target::rtt_init_defmt!();
}

#[cfg(test)]
#[embedded_test::tests(setup=crate::setup_log())]
pub mod data_tests {
    // This is here so we get the entry point for embassy properly
    extern crate embassy_stm32;

    use fsm::commons::{EmergencyChannel, EventChannel};
    use fsm::commons::data::{Event, PriorityEventPubSub};

    static EVENT_CHANNEL: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
    static EMERGENCY_CHANNEL: static_cell::StaticCell<EmergencyChannel> =
        static_cell::StaticCell::new();

    #[test]
    async fn test() {
        let event_channel = EVENT_CHANNEL.init(EventChannel::new());
        let emergency_channel = EMERGENCY_CHANNEL.init(EmergencyChannel::new());

        let expected_events: [Event; 4] = [Event::NoEvent, Event::SystemCheckSuccess, Event::Activate, Event::Emergency];

        let mut priority_event_pub_sub = PriorityEventPubSub::new(
            event_channel.publisher().unwrap(),
            event_channel.subscriber().unwrap(),
            emergency_channel.publisher().unwrap(),
            emergency_channel.subscriber().unwrap()
        );

        for event in expected_events {
            priority_event_pub_sub.add_event(&event).await;
        }

        assert_eq!(priority_event_pub_sub.get_event().await, Event::Emergency);

        for i in 0..expected_events.len() - 1 {
            assert_eq!(priority_event_pub_sub.get_event().await, expected_events[i]);
        }
    }
}