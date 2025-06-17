//! Types for the ethernet logic

use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::GenericPhy;
use embassy_stm32::peripherals::ETH;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::pubsub::PubSubChannel;
use embassy_sync::pubsub::Publisher;
use embassy_sync::pubsub::Subscriber;
use lib::config::Command;
use lib::Datapoint;

/// todo: docs
const RX_BUFFER_SIZE: usize = 8192;
/// todo: docs
const TX_BUFFER_SIZE: usize = 32768;

/// max references
const CAP: usize = 8;
/// max number of subscribers
const SUBS: usize = 4;
/// max number of publishers
const PUBS: usize = 1;
///
const TX_CAP: usize = 1024;

/// pub-sub channel for gs->pod
pub type GsToPodChannel = PubSubChannel<NoopRawMutex, GsToPodMessage, CAP, SUBS, PUBS>;
/// ground station -> pod publisher
pub type GsToPodPublisher<'a> = Publisher<'a, NoopRawMutex, GsToPodMessage, CAP, SUBS, PUBS>;
/// ground station -> pod subscriber
pub type GsToPodSubscriber<'a> = Subscriber<'a, NoopRawMutex, GsToPodMessage, CAP, SUBS, PUBS>;

/// pub-sub channel for pod->gs
pub type PodToGsChannel = Channel<NoopRawMutex, PodToGsMessage, CAP>;
/// pod -> gs publisher
pub type PodToGsPublisher<'a> =
    embassy_sync::channel::Sender<'a, NoopRawMutex, crate::gs_master::PodToGsMessage, TX_CAP>;
/// pod -> gs subscriber
pub type PodToGsSubscriber<'a> =
    embassy_sync::channel::Receiver<'a, NoopRawMutex, crate::gs_master::PodToGsMessage, TX_CAP>;

/// TODO: docs
pub type EthDevice = Ethernet<'static, ETH, GenericPhy>;

/// Struct used to store the communication channels
pub struct Comms {
    pub(crate) rx_channel: GsToPodChannel,
    pub(crate) tx_channel: PodToGsChannel,
}

impl Comms {
    pub fn new() -> Self {
        Self {
            rx_channel: GsToPodChannel::new(),
            tx_channel: PodToGsChannel::new(),
        }
    }
}

/// Struct used to represent a message from the ground station to the pod
#[derive(Clone, Debug, defmt::Format)]
pub struct GsToPodMessage {
    /// The command sent
    pub command: Command,
}

impl GsToPodMessage {
    /// TODO: docs
    pub(crate) const SIZE: usize = 20;

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
