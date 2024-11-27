//! This module contains constants and types that are shared between the FSMs.

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::pubsub::{PubSubChannel, Publisher, Subscriber};
use crate::commons::data::Event;

pub(crate) mod data;
pub(crate) mod macros;
pub(crate) mod traits;

// Publishers and subscribers for the channel used to broadcast events to all the FSMs.
// Max 32 events, 6 subscribers, 7 publishers(FSMs and main).
const MAX_EVENTS: usize = 32;
const MAX_EMERGENCY_EVENTS: usize = 4;
const SUBSCRIBERS: usize = 6;
const PUBLISHERS: usize = 7;

// Types for the two channels used for broadcasting events to each FSM and for publishers and subscribers of the channels.
pub type EventChannel = PubSubChannel<NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;
pub type EmergencyChannel = PubSubChannel<NoopRawMutex, Event, MAX_EMERGENCY_EVENTS, SUBSCRIBERS, PUBLISHERS>;

pub type PublisherChannel = Publisher<'static, NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;
pub type SubscriberChannel = Subscriber<'static, NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;

pub type PublisherEmergency = Publisher<'static, NoopRawMutex, Event, MAX_EMERGENCY_EVENTS, SUBSCRIBERS, PUBLISHERS>;
pub type SubscriberEmergency = Subscriber<'static, NoopRawMutex, Event, MAX_EMERGENCY_EVENTS, SUBSCRIBERS, PUBLISHERS>;