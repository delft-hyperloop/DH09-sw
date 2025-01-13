//! This module contains constants and types that are shared between the FSMs.

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::pubsub::PubSubChannel;
use embassy_sync::pubsub::Publisher;
use embassy_sync::pubsub::Subscriber;

pub use crate::commons::data::Event;

pub mod data;
pub(crate) mod macros;
pub mod traits;

// Publishers and subscribers for the channel used to broadcast events to all
// the FSMs. Max 32 events, 6 subscribers, 7 publishers(FSMs and main).
const MAX_EVENTS: usize = 32;
const MAX_EMERGENCY_EVENTS: usize = 4;
const SUBSCRIBERS: usize = 16;
const PUBLISHERS: usize = 16;

/// Type alias for the `PubSubChannel` used for the normal event channel.
pub type EventChannel = PubSubChannel<NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;

/// Type alias for the `PubSubChannel` used for the emergency event channel.
pub type EmergencyChannel =
    PubSubChannel<NoopRawMutex, Event, MAX_EMERGENCY_EVENTS, SUBSCRIBERS, PUBLISHERS>;

/// Type alias for the normal event channel `Publisher`.
pub type PublisherChannel =
    Publisher<'static, NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;
/// Type alias for the normal event channel `Subscriber`.
pub type SubscriberChannel =
    Subscriber<'static, NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;

/// Type alias for the emergency channel `Publisher`.
pub type PublisherEmergency =
    Publisher<'static, NoopRawMutex, Event, MAX_EMERGENCY_EVENTS, SUBSCRIBERS, PUBLISHERS>;
/// Type alias for the emergency channel `Subscriber`.
pub type SubscriberEmergency =
    Subscriber<'static, NoopRawMutex, Event, MAX_EMERGENCY_EVENTS, SUBSCRIBERS, PUBLISHERS>;
