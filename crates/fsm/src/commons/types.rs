//! This module contains types used in the crate

use core::fmt::Debug;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::priority_channel::PriorityChannel;
use embassy_sync::priority_channel::Sender;
use embassy_sync::priority_channel::Receiver;
use crate::commons::Event;

/// Maximum number of events on the channel
const MAX_EVENTS: usize = 32;

/// Type alias for the `PriorityChannel` used for the normal event channel.
pub type EventChannel = PriorityChannel<NoopRawMutex, Event, embassy_sync::priority_channel::Min, MAX_EVENTS>;

/// Type alias for the event channel sender.
pub type EventSender = Sender<'static, NoopRawMutex, Event, embassy_sync::priority_channel::Min, MAX_EVENTS>;
/// Type alias for the event channel receiver.
pub type EventReceiver = Receiver<'static, NoopRawMutex, Event, embassy_sync::priority_channel::Min, MAX_EVENTS>;
