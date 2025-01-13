//! This module contains enums and structs shared among the FSMs, as well as
//! their implementations.

use defmt::Format;
use embassy_sync::pubsub::WaitResult;

use crate::commons::PublisherChannel;
use crate::commons::PublisherEmergency;
use crate::commons::SubscriberChannel;
use crate::commons::SubscriberEmergency;

/// Enum representing different types of events that the FSMs should handle.
#[derive(Clone, PartialEq, Eq, Debug, Copy, Format)]
pub enum Event {
    /// No event happened
    NoEvent,
    /// Stops the sub-FSMs
    StopSubFSMs,
    /// Stops the FSM
    StopFSM,
    /// Emergency happened
    Emergency,
    /// System was checked successfully
    SystemCheckSuccess,
    /// Activates the pod
    Activate,
    /// Pod should start charging
    Charge,
    /// Pod should stop charging
    StopCharge,
    /// Pod should enter the `operating` state which starts the sub-FSMs
    Operate,
    /// Pod should enter the `Demo` state
    Demo,
    /// Pod should now cruise
    Cruise,
    /// Pod should brake
    Brake,
    /// Pod should shut down
    ShutDown,
    /// High Voltage should turn on
    HighVoltageOn,
    /// High Voltage should turn off
    HighVoltageOff,
    /// Propulsion should turn on
    PropulsionOn,
    /// Propulsion should turn off
    PropulsionOff,
    /// Pod should start accelerating
    Accelerate {
        /// The velocity profile used to accelerate
        velocity_profile: u8, // TODO: Change to actual velocity profile
    },
    /// Levitation should turn on
    LevitationOn,
    /// Levitation should turn off
    LevitationOff,
}

/// Struct used for publishing and polling events from each channel.
pub struct PriorityEventPubSub {
    pub(crate) event_channel_publisher: PublisherChannel,
    pub(crate) event_channel_subscriber: SubscriberChannel,
    pub(crate) emergency_channel_publisher: PublisherEmergency,
    pub(crate) emergency_channel_subscriber: SubscriberEmergency,
}

impl core::fmt::Debug for PriorityEventPubSub {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "hello:)")
    }
}

impl PriorityEventPubSub {
    /// Constructor method for the PriorityEventPubSub struct
    pub fn new(
        event_channel_publisher: PublisherChannel,
        event_channel_subscriber: SubscriberChannel,
        emergency_channel_publisher: PublisherEmergency,
        emergency_channel_subscriber: SubscriberEmergency,
    ) -> Self {
        Self {
            event_channel_subscriber,
            event_channel_publisher,
            emergency_channel_subscriber,
            emergency_channel_publisher,
        }
    }

    /// Fetches an event from the two channels, prioritizing the emergency
    /// channel.
    ///
    /// This method first checks for an event on the emergency channel. If an
    /// event is present, it will be returned immediately. If no event is
    /// available on the emergency channel, it then checks the normal
    /// channel for an event.
    ///
    /// # Returns
    /// - `Event`: If an event is found on either channel, the event is
    ///   returned.
    /// - `Event::NoEvent`: If the subscriber missed any messages
    pub async fn get_event(&mut self) -> Event {
        let event;
        if self.emergency_channel_subscriber.available() != 0 {
            event = self.emergency_channel_subscriber.next_message().await;
        } else if !self.event_channel_subscriber.available() != 0 {
            event = self.event_channel_subscriber.next_message().await;
        } else {
            return Event::NoEvent;
        }
        match event {
            WaitResult::Message(received_event) => received_event,
            WaitResult::Lagged(amount) => {
                // TODO: Problem? This means that the subscriber missed {amount} messages
                defmt::warn!("Missed {} messages on fsm channel", amount);
                Event::NoEvent
            }
        }
    }

    /// Adds an event to one of the channels.
    ///
    /// This method checks if the event provided is an emergency. In that case,
    /// it publishes the event on the emergency channel, otherwise it
    /// publishes it on the normal broadcasting channel.
    pub async fn add_event(&self, event: &Event) {
        if *event == Event::Emergency {
            self.emergency_channel_publisher
                .publish(event.clone())
                .await;
        } else {
            self.event_channel_publisher.publish(event.clone()).await;
        }
    }
}
