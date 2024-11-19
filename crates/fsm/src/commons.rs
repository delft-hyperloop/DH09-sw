use alloc::sync::Arc;
use core::cmp::PartialEq;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::pubsub::{PubSubChannel, Publisher, Subscriber, WaitResult};
use crate::commons::Event::NoEvent;

#[derive(Clone, PartialEq)]
pub enum Event {
    NoEvent,
    Emergency,
    SystemCheckSuccess,
    Activate,
    Charge,
    StopCharge,
    Operate,        // Event to enter the big operating state
    Demo,
    Cruise,
    Brake,
    ShutDown,
    Quit,
    HighVoltageOn,
    HighVoltageOff,
    Accelerate {
        velocity_profile: u8 // TODO: Change to actual velocity profile
    },
    PropulsionOn,
    PropulsionOff,
    PropulsionRunning,
    LevitationOn,
    LevitationOff,
}

pub trait Runner {
    fn get_pub_sub_channel(&mut self) -> &mut Arc<PriorityEventPubSub>;

    fn handle_events(&mut self, event: Event);

    async fn run(&mut self) {
        loop {
            let event = Arc::get_mut(self.get_pub_sub_channel()).unwrap().poll().await;
            self.handle_events(event);
        }
    }
}

#[macro_export]
macro_rules! impl_runner_get_sub_channel {
    ($fsm_struct:ident) => {
        impl Runner for $fsm_struct {
            fn get_pub_sub_channel(&mut self) -> &mut Arc<PriorityEventPubSub> {
                &mut self.priority_event_pub_sub
            }

            fn handle_events(&mut self, event: Event) {
                Self::handle(self, event);
            }
        }
    };
}

pub trait Transition<T> {
    fn entry_method(&mut self) -> fn();

    fn exit_method(&mut self) -> fn();

    fn set_state(&mut self, new_state: T);

    fn transition(&mut self, state: T) {
        // Gets the exit method associated with the current state
        let exit_method = self.exit_method();
        exit_method();

        // Transitions to new state
        self.set_state(state);

        // Calls the entry method for the new state
        let entry_method = self.entry_method();
        entry_method();
    }
}

#[macro_export]
macro_rules! impl_transition {
    ($fsm_struct:ident, $fsm_states: ident) => {
        impl Transition<$fsm_states> for $fsm_struct {
            fn entry_method(&mut self) -> fn() {
                ENTRY_FUNCTION_MAP[self.state as usize]
            }

            fn exit_method(&mut self) -> fn() {
                EXIT_FUNCTION_MAP[self.state as usize]
            }

            fn set_state(&mut self, new_state: $fsm_states) {
                self.state = new_state;
            }
        }
    };
}

// Publishers and subscribers for the channel used to broadcast events to all the FSMs.
// Max 32 events, 6 subscribers, 7 publishers(FSMs and main).
const MAX_EVENTS: usize = 32;
const MAX_EMERGENCY_EVENTS: usize = 4;
const SUBSCRIBERS: usize = 6;
const PUBLISHERS: usize = 7;

pub type EventChannel = PubSubChannel<NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;
pub type EmergencyChannel = PubSubChannel<NoopRawMutex, Event, MAX_EMERGENCY_EVENTS, SUBSCRIBERS, PUBLISHERS>;

pub type PublisherChannel = Publisher<'static, NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;
pub type SubscriberChannel = Subscriber<'static, NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;

pub type PublisherEmergency = Publisher<'static, NoopRawMutex, Event, MAX_EMERGENCY_EVENTS, SUBSCRIBERS, PUBLISHERS>;
pub type SubscriberEmergency = Subscriber<'static, NoopRawMutex, Event, MAX_EMERGENCY_EVENTS, SUBSCRIBERS, PUBLISHERS>;

pub struct PriorityEventPubSub {
    event_channel_publisher: PublisherChannel,
    event_channel_subscriber: SubscriberChannel,
    emergency_channel_publisher: PublisherEmergency,
    emergency_channel_subscriber: SubscriberEmergency,
}

impl PriorityEventPubSub {
    pub fn new(
        event_channel_publisher: PublisherChannel,
        event_channel_subscriber: SubscriberChannel,
        emergency_channel_publisher: PublisherEmergency,
        emergency_channel_subscriber: SubscriberEmergency,
    ) -> Self {
      Self{
          event_channel_subscriber,
          event_channel_publisher,
          emergency_channel_subscriber,
          emergency_channel_publisher,
      }
    }

    pub async fn poll(&mut self) -> Event {
        let event;
        if self.emergency_channel_subscriber.available() != 0 {
            event = self.emergency_channel_subscriber.next_message().await;
        } else if !self.event_channel_subscriber.available() != 0 {
            event = self.event_channel_subscriber.next_message().await;
        } else {
            return NoEvent;
        }
        match event {
            WaitResult::Message(received_event) => received_event,
            WaitResult::Lagged(_amount) => {
                // TODO: Problem? This means that the subscriber missed {amount} messages
                NoEvent
            },
        }
    }

    pub async fn add_event(&self, event: &Event) {
        if *event == Event::Emergency {
            self.emergency_channel_publisher.publish(event.clone()).await;
        } else {
            self.event_channel_publisher.publish(event.clone()).await;
        }
    }
}

#[macro_export]
macro_rules! define_fsm {
    ($fsm_name:ident) => {
        $fsm_name::new(PriorityEventPubSub::new(
            event_channel.publisher().unwrap(),
            event_channel.subscriber().unwrap(),
            emergency_channel.publisher().unwrap(),
            emergency_channel.subscriber().unwrap(),
        ))
    };
}
