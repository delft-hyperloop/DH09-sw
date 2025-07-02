//! This module contains types used in the crate

use core::fmt::Debug;
use core::fmt::Formatter;

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::priority_channel::PriorityChannel;
use embassy_sync::priority_channel::Receiver;
use embassy_sync::priority_channel::Sender;
use heapless::binary_heap::Min;

use crate::{Datapoint, Event};

/// Maximum number of events on the channel
const MAX_EVENTS: usize = 32;

/// Type alias for the `PriorityChannel` used for the normal event channel.
pub type EventChannel =
    PriorityChannel<NoopRawMutex, Event, embassy_sync::priority_channel::Min, MAX_EVENTS>;

/// Type alias for the event channel sender.
#[derive(Copy, Clone)]
pub struct EventSender(
    Sender<'static, NoopRawMutex, Event, embassy_sync::priority_channel::Min, MAX_EVENTS>,
);
/// Type alias for the event channel receiver.
#[derive(Copy, Clone)]
pub struct EventReceiver(
    Receiver<'static, NoopRawMutex, Event, embassy_sync::priority_channel::Min, MAX_EVENTS>,
);

impl EventReceiver {
    /// Wrapper method for the `receive` method of the `EventReceiver`
    pub async fn receive(&self) -> Event {
        self.0.receive().await
    }
}

impl EventSender {
    /// Wrapper method for the `send` method of the `EventSender`
    pub async fn send(&self, event: Event) {
        self.0.send(event).await
    }
}

impl Debug for EventSender {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "EventSender {{ ... }}")
    }
}

impl Debug for EventReceiver {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "EventReceiver {{ ... }}")
    }
}

impl From<Receiver<'static, NoopRawMutex, Event, embassy_sync::priority_channel::Min, MAX_EVENTS>>
    for EventReceiver
{
    fn from(receiver: Receiver<'static, NoopRawMutex, Event, Min, MAX_EVENTS>) -> Self {
        EventReceiver(receiver)
    }
}

impl From<Sender<'static, NoopRawMutex, Event, embassy_sync::priority_channel::Min, MAX_EVENTS>>
    for EventSender
{
    fn from(sender: Sender<'static, NoopRawMutex, Event, Min, MAX_EVENTS>) -> Self {
        EventSender(sender)
    }
}
