//! Types for the ethernet logic

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::pubsub::{PubSubChannel, Publisher, Subscriber};
use lib::config::Command;
use lib::Datapoint;

/// todo: docs
const RX_BUFFER_SIZE: usize = 8192;
/// todo: docs
const TX_BUFFER_SIZE: usize = 32768;

/// max references
const RX_CAP: usize = 8;
/// max number of subscribers
const RX_SUBS: usize = 4;
/// max number of publishers
const RX_PUBS: usize = 1;
/// pub-sub channel for gs->pod
type RxChannel = PubSubChannel<NoopRawMutex, GsToPodMessage, RX_CAP, RX_SUBS, RX_PUBS>;
/// ground station -> pod publisher
type RxPublisher<'a> = Publisher<'a, NoopRawMutex, GsToPodMessage, RX_CAP, RX_SUBS, RX_PUBS>;
/// RxSubscriber
pub type RxSubscriber<'a> = Subscriber<'a, NoopRawMutex, GsToPodMessage, RX_CAP, RX_SUBS, RX_PUBS>;


/// Struct used to represent a message from the ground station to the pod
#[derive(Clone, Debug, defmt::Format)]
pub struct GsToPodMessage {
    /// The command sent
    pub command: Command,
}

impl GsToPodMessage {
    /// todo: docs
    const SIZE: usize = 20;

    /// Reads from the buffer
    /// - `buf`: the buffer
    pub fn read_from_buf(buf: &[u8; Self::SIZE]) -> Self {
        let command = Command::from_bytes(buf);

        Self { command }
    }
}

/// Struct for the datapoints sent from the pod to the ground station
#[derive(Clone, Debug)]
pub struct PodToGsMessage {
    /// The datapoint being sent
    pub dp: Datapoint,
}