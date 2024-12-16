#[cfg(test)]
pub mod tests {
    extern crate alloc;

    use alloc::vec;

    use static_cell::StaticCell;

    use crate::commons::data::Event;
    use crate::commons::data::PriorityEventPubSub;
    use crate::commons::EmergencyChannel;
    use crate::commons::EventChannel;

    fn setup() -> PriorityEventPubSub {
        static EVENT_CHANNEL: StaticCell<EventChannel> = StaticCell::new();
        static EMERGENCY_CHANNEL: StaticCell<EmergencyChannel> = StaticCell::new();
        let event_channel = EVENT_CHANNEL.init(EventChannel::new());
        let emergency_channel = EMERGENCY_CHANNEL.init(EmergencyChannel::new());

        PriorityEventPubSub::new(
            event_channel.publisher().unwrap(),
            event_channel.subscriber().unwrap(),
            emergency_channel.publisher().unwrap(),
            emergency_channel.subscriber().unwrap(),
        )
    }

    #[test]
    async fn test_priority_event_pub_sub() {
        let mut priority_event_pub_sub = setup();

        let expected_events = vec![
            Event::HighVoltageOn,
            Event::LevitationOn,
            Event::LevitationOff,
            Event::HighVoltageOff,
        ];

        for event in expected_events.clone() {
            priority_event_pub_sub.add_event(&event).await;
        }

        for event in expected_events {
            let polled_event = priority_event_pub_sub.get_event();
            assert_eq!(event, polled_event);
        }
    }

    #[test]
    async fn test_emergency_priority_event_pub_sub() {
        let mut priority_event_pub_sub = setup();

        let expected_events = vec![Event::NoEvent, Event::SystemCheckSuccess, Event::Activate];

        for event in expected_events.clone() {
            priority_event_pub_sub.add_event(&event).await;
        }
        priority_event_pub_sub.add_event(&Event::Emergency).await;

        let event = priority_event_pub_sub.get_event();
        assert_eq!(Event::Emergency, event);

        for event in expected_events {
            let polled_event = priority_event_pub_sub.get_event();
            assert_eq!(event, polled_event);
        }
    }
}
