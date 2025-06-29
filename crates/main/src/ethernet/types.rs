//! Types for the ethernet logic

use core::fmt::{Debug, Formatter};
use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::GenericPhy;
use embassy_stm32::peripherals::ETH;
use embassy_stm32::peripherals::PA1;
use embassy_stm32::peripherals::PA2;
use embassy_stm32::peripherals::PA7;
use embassy_stm32::peripherals::PB11;
use embassy_stm32::peripherals::PB12;
use embassy_stm32::peripherals::PB13;
use embassy_stm32::peripherals::PC1;
use embassy_stm32::peripherals::PC4;
use embassy_stm32::peripherals::PC5;
use embassy_stm32::Peri;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::pubsub::PubSubChannel;
use embassy_sync::pubsub::Publisher;
use embassy_sync::pubsub::Subscriber;
use embassy_time::Duration;
use lib::config::Command;
use lib::Datapoint;

/// if nothing is sent over tcp for [timeout], send an RST and close the
/// connection. keep alive will send a TCP_KEEP_ALIVE frame every [duration]
/// milliseconds.
pub const SOCKET_KEEP_ALIVE: Duration = Duration::from_millis(300);

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
pub type PodToGsChannel = Channel<NoopRawMutex, PodToGsMessage, TX_CAP>;
/// pod -> gs publisher
pub type PodToGsPublisher<'a> =
    embassy_sync::channel::Sender<'a, NoopRawMutex, PodToGsMessage, TX_CAP>;
/// pod -> gs subscriber
pub type PodToGsSubscriber<'a> =
    embassy_sync::channel::Receiver<'a, NoopRawMutex, PodToGsMessage, TX_CAP>;

/// TODO: docs
pub type EthDevice = Ethernet<'static, ETH, GenericPhy>;

/// Struct used to store the communication channels between the GsMaster and the
/// outside
pub struct GsComms {
    /// The channel used to receive commands from the pod
    pub(crate) rx_channel: GsToPodChannel,
    /// The channel used to send datapoints to the pod
    pub(crate) tx_channel: PodToGsChannel,
}

impl Debug for GsComms {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // idk wtf to write other than this good luck bro
        write!(f, "GsComms")
    }
}

impl Default for GsComms {
    fn default() -> Self {
        Self::new()
    }
}

impl GsComms {
    /// Constructor for an instance of the Comms struct
    pub fn new() -> Self {
        Self {
            rx_channel: GsToPodChannel::new(),
            tx_channel: PodToGsChannel::new(),
        }
    }

    /// Gives a subscriber object for the PodToGsChannel
    pub fn tx_receiver(&self) -> PodToGsSubscriber {
        self.tx_channel.receiver()
    }

    /// Give a publisher object for the PodToGsChannel
    pub fn tx_publisher(&self) -> PodToGsPublisher {
        self.tx_channel.sender()
    }

    /// Gives a publisher object for the GsToPodChannel
    pub fn rx_publisher(&self) -> GsToPodPublisher {
        self.rx_channel.publisher().unwrap()
    }

    /// Gives a receiver object for the GsToPodChannel
    pub fn rx_receiver(&self) -> GsToPodSubscriber {
        self.rx_channel.subscriber().unwrap()
    }
}

/// todo: docs
pub(crate) const RX_BUFFER_SIZE: usize = 8192;
/// todo: docs
pub(crate) const TX_BUFFER_SIZE: usize = 32768;

/// Struct used to represent a message from the ground station to the pod
#[derive(Clone, Debug, defmt::Format, Copy)]
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
#[derive(Clone, Debug, Copy)]
pub struct PodToGsMessage {
    /// The datapoint being sent
    pub dp: Datapoint,
}

/// The pins used for ethernet
pub struct EthPeripherals {
    /// todo
    pub eth: Peri<'static, ETH>,
    /// todo
    pub pa1: Peri<'static, PA1>,
    /// todo
    pub pa2: Peri<'static, PA2>,
    /// todo
    pub pc1: Peri<'static, PC1>,
    /// todo
    pub pa7: Peri<'static, PA7>,
    /// todo
    pub pc4: Peri<'static, PC4>,
    /// todo
    pub pc5: Peri<'static, PC5>,
    /// todo
    pub pb12: Peri<'static, PB12>,
    /// todo
    pub pb13: Peri<'static, PB13>,
    /// todo
    pub pb11: Peri<'static, PB11>,
}

impl Debug for EthPeripherals {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "EthPeripherals")
    }
}