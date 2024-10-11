//! Module that deals with communication to the CAN bus.
//!
//! The main type is [`CanInterface`], which is used to
//! expose an interface to the CAN implementation.

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::can::{Can, CanRx, CanTx};
use embassy_sync::{
    blocking_mutex::raw::NoopRawMutex,
    priority_channel::{self, PriorityChannel},
    pubsub::{PubSubChannel, Publisher, Subscriber},
};
use embassy_time::Timer;
use static_cell::StaticCell;

#[derive(Debug, Clone)]
pub struct CanEnvelope {
    envelope: embassy_stm32::can::frame::FdEnvelope,
}

impl core::cmp::PartialEq for CanEnvelope {
    fn eq(&self, other: &Self) -> bool {
        self.envelope.frame.id() == other.envelope.frame.id()
    }
}

impl core::cmp::PartialOrd for CanEnvelope {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.envelope
            .frame
            .id()
            .partial_cmp(&other.envelope.frame.id())
    }
}

impl core::cmp::Eq for CanEnvelope {}

impl core::cmp::Ord for CanEnvelope {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.envelope.frame.id().cmp(&other.envelope.frame.id())
    }
}

const CAN_RX_CAPACITY: usize = 4;
const CAN_RX_SUBSCRIBERS: usize = 4;
const CAN_RX_PUBLISHERS: usize = 1;

type CanRxChannel = PubSubChannel<
    NoopRawMutex,
    CanEnvelope,
    CAN_RX_CAPACITY,
    CAN_RX_SUBSCRIBERS,
    CAN_RX_PUBLISHERS,
>;
type CanRxSubscriber<'a> = Subscriber<
    'a,
    NoopRawMutex,
    CanEnvelope,
    CAN_RX_CAPACITY,
    CAN_RX_SUBSCRIBERS,
    CAN_RX_PUBLISHERS,
>;
type CanRxPublisher<'a> = Publisher<
    'a,
    NoopRawMutex,
    CanEnvelope,
    CAN_RX_CAPACITY,
    CAN_RX_SUBSCRIBERS,
    CAN_RX_PUBLISHERS,
>;

/// Task that listens for CAN messages sent over the CAN bus and forwards them to the RX channel.
///
/// Tasks interested in receiving CAN messages should subscribe to the RX channel, which they can do
/// through [`CanInterface::new_subscriber`].
#[embassy_executor::task]
async fn can_rx_task(mut can: CanRx<'static>, publisher: CanRxPublisher<'static>) -> ! {
    let mut error_counter: usize = 0;
    loop {
        match can.read_fd().await {
            Ok(envelope) => {
                publisher.publish(CanEnvelope { envelope }).await;
            }
            Err(e) => {
                if error_counter < 10 || error_counter % 2500 == 0 {
                    error!(
                        "[CAN] Error reading from CAN bus (#{}): {:?}",
                        error_counter, e
                    );
                }
                Timer::after_millis(500).await;
                error_counter = error_counter.wrapping_add(1);
            }
        }
    }
}

const CAN_TX_CAPACITY: usize = 32;
type CanTxChannelKind = heapless::binary_heap::Min;
type CanTxChannel = PriorityChannel<NoopRawMutex, CanEnvelope, CanTxChannelKind, CAN_TX_CAPACITY>;
type CanTxSender<'a> =
    priority_channel::Sender<'a, NoopRawMutex, CanEnvelope, CanTxChannelKind, CAN_TX_CAPACITY>;
type CanTxReceiver<'a> =
    priority_channel::Receiver<'a, NoopRawMutex, CanEnvelope, CanTxChannelKind, CAN_TX_CAPACITY>;

/// Task that sends CAN envelopes received from the TX channel over the CAN bus.
#[embassy_executor::task]
async fn can_tx_task(
    mut can: CanTx<'static>,
    // mut retransmit_sender: CanTxSender<'static>,
    rx: CanTxReceiver<'static>,
) -> ! {
    loop {
        let envelope = rx.receive().await;
        let frame = can.write_fd(&envelope.envelope.frame).await;
        match frame {
            None => {
                // Success
            }
            Some(e) => {
                // retransmit_sender.send(envelope).await;
            }
        }
    }
}

pub struct CanInterface {
    rx_channel: CanRxChannel,
    tx_channel: CanTxChannel,
}

impl CanInterface {
    /// Initializes the CAN interface.
    ///
    /// This function should be called once at the beginning of the program.
    ///
    /// It takes in the CAN peripheral, which should be initialized and configured
    /// before calling this function, and a spawner, which is used to spawn the
    /// RX and TX tasks.
    pub fn new(can: Can<'static>, spawner: &Spawner) -> &'static Self {
        static CAN_INTERFACE: StaticCell<CanInterface> = StaticCell::new();

        let rx_channel = CanRxChannel::new();
        let tx_channel = CanTxChannel::new();

        let interface = CAN_INTERFACE.init(Self {
            rx_channel,
            tx_channel,
        });

        let (can_tx, can_rx, _props) = can.split();
        let publisher = unwrap!(interface.rx_channel.publisher());
        let receiver = interface.tx_channel.receiver();

        unwrap!(spawner.spawn(can_rx_task(can_rx, publisher)));
        unwrap!(spawner.spawn(can_tx_task(can_tx, receiver)));

        interface
    }

    /// Adds a new subscriber to the RX channel.
    ///
    /// The subscriber will be notified about all the
    /// CAN messages received from the CAN bus.
    pub fn new_subscriber(&self) -> CanRxSubscriber<'_> {
        unwrap!(self.rx_channel.subscriber())
    }
}
