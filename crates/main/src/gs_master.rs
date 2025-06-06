//! The code that interacts with the ground station.
//!
//! The main type of this module is the [`GsMaster`],
//! which is the "interface" to the outside.
//!
//! It exposes the following functions:
//! - [`GsMaster::subscribe`] to subscribe to messages received from the GS.
//! - [`GsMaster::transmitter`] to get a sender you can use to transmit messages
//!   to the GS.
//!
//! The messages themselves are of the types [`PodToGsMessage`]
//! and [`GsToPodMessage`], depending on how they flow.
//!
//! The [`GsToPodMessage`] contains an FSM [`lib::utils::data::Event`],
//! which may be [`lib::utils::data::Event::NoEvent`] in case it
//! shouldn't be processed. In other cases, it is forwarded to the FSM.
//!
//! The [`PodToGsMessage`] may contain some very basic logging information,
//! as most of the logging happens through the logging PCB in order to let the
//! main PCB run the FSM.

#![allow(missing_copy_implementations)]
#![allow(missing_debug_implementations)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::Ipv4Address;
use embassy_net::Stack;
use embassy_net::StackResources;
use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::GenericPhy;
use embassy_stm32::peripherals::ETH;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::PubSubChannel;
use embassy_sync::pubsub::Publisher;
use embassy_sync::pubsub::Subscriber;
use embassy_sync::signal::Signal;
use embassy_time::Instant;
use embassy_time::Timer;
use embedded_io_async::Read;
use embedded_io_async::ReadExactError;
use embedded_io_async::Write;
use static_cell::StaticCell;

type GsCommsLayerImpl = EthernetGsCommsLayer;

///
pub struct GsMaster<C: GsCommsLayer> {
    comms: C,
}

impl GsMaster<GsCommsLayerImpl> {
    /// Constructor for the `GsMaster`
    pub async fn new<I>(comms: I, spawner: Spawner) -> &'static GsMaster<GsCommsLayerImpl>
    where
        I: GsCommsLayerInitializable<CommsLayer = GsCommsLayerImpl>,
    {
        let comms = comms.init(spawner).await;
        static GS_MASTER: StaticCell<GsMaster<GsCommsLayerImpl>> = StaticCell::new();
        let gs_master = GS_MASTER.init(GsMaster { comms });
        gs_master
    }
}

impl<C: GsCommsLayer> GsMaster<C> {
    /// Returns a subscriber object for communicating with the ground station
    pub fn subscribe(&self) -> RxSubscriber<'_> {
        self.comms.subscribe()
    }

    /// Returns a sender object for communicating with the ground station
    pub fn transmitter(&self) -> TxSender<'_> {
        self.comms.transmitter()
    }
}

/// Struct used to represent a message from the ground station to the pod
#[derive(Clone, Debug, defmt::Format)]
pub struct GsToPodMessage {
    /// The command sent
    pub command: Command,
}

impl GsToPodMessage {
    const SIZE: usize = 20;

    /// Reads from the buffer
    /// - `buf`: the buffer
    pub fn read_from_buf(buf: &[u8; Self::SIZE]) -> Self {
        let command = Command::from_bytes(buf);

        Self { command }
    }
}

use core::cmp::Ordering;

use defmt::Formatter;
use lib::config;
use lib::config::Command;
use lib::config::Datatype;
use lib::config::COMMAND_HASH;
use lib::config::CONFIG_HASH;
use lib::config::DATA_HASH;
use lib::config::EVENTS_HASH;
use lib::Datapoint;

/// Struct for the datapoints sent from the pod to the ground station
#[derive(Clone, Debug)]
pub struct PodToGsMessage {
    /// The datapoint being sent
    pub dp: Datapoint,
}

/// Trait for a struct that communicates with the ground station
pub trait GsCommsLayer {
    /// Subscriber
    fn subscribe(&self) -> RxSubscriber<'_>;
    /// Transmitter
    fn transmitter(&self) -> TxSender<'_>;
}

///
pub trait GsCommsLayerInitializable {
    ///
    type CommsLayer: GsCommsLayer;

    /// Initializes the comms with the GS
    async fn init(self, spawner: Spawner) -> Self::CommsLayer;
}

const RX_BUFFER_SIZE: usize = 8192;
const TX_BUFFER_SIZE: usize = 32768;

struct CommsBuffers {
    rx: [u8; RX_BUFFER_SIZE],
    tx: [u8; TX_BUFFER_SIZE],
}

struct CommsCore {
    rx_channel: RxChannel,
    tx_channel: TxChannel,
}

///
pub struct EthernetGsCommsLayer {
    cc: &'static CommsCore,
}

impl GsCommsLayer for EthernetGsCommsLayer {
    fn subscribe(&self) -> RxSubscriber<'_> {
        unwrap!(self.cc.rx_channel.subscriber())
    }

    fn transmitter(&self) -> TxSender<'_> {
        self.cc.tx_channel.sender()
    }
}

type EthDevice = Ethernet<'static, ETH, GenericPhy>;

///
pub struct EthernetGsCommsLayerInitializer {
    seed: u64,
    device: EthDevice,
    config: embassy_net::Config,
}

impl EthernetGsCommsLayerInitializer {
    /// Constructor for the `EthernetGsCommsLayerInitializer` struct
    pub fn new(device: EthDevice, config: embassy_net::Config) -> Self {
        Self {
            seed: 0x1234567890ABCDEF,
            device,
            config,
        }
    }
}

const TX_CAP: usize = 1024;
type TxChannel = Channel<NoopRawMutex, PodToGsMessage, TX_CAP>;
type TxReceiver<'a> = embassy_sync::channel::Receiver<'a, NoopRawMutex, PodToGsMessage, TX_CAP>;
/// TxSender
pub type TxSender<'a> = embassy_sync::channel::Sender<'a, NoopRawMutex, PodToGsMessage, TX_CAP>;

const RX_CAP: usize = 8;
const RX_SUBS: usize = 4;
const RX_PUBS: usize = 1;
type RxChannel = PubSubChannel<NoopRawMutex, GsToPodMessage, RX_CAP, RX_SUBS, RX_PUBS>;
type RxPublisher<'a> = Publisher<'a, NoopRawMutex, GsToPodMessage, RX_CAP, RX_SUBS, RX_PUBS>;
/// RxSubscriber
pub type RxSubscriber<'a> = Subscriber<'a, NoopRawMutex, GsToPodMessage, RX_CAP, RX_SUBS, RX_PUBS>;

type ReconnectSignal = Signal<CriticalSectionRawMutex, ()>;
type ConnectedSignal = Signal<CriticalSectionRawMutex, ()>;

#[embassy_executor::task]
async fn rx_task(
    sock: &'static Mutex<NoopRawMutex, TcpSocket<'static>>,
    publisher: RxPublisher<'static>,
    rs: &'static ReconnectSignal,
    cs: &'static ConnectedSignal,
) -> ! {
    let mut buf = [0; GsToPodMessage::SIZE];

    loop {
        debug!("Attempting to read");

        if rs.signaled() {
            let _ = cs.wait().await;
        }

        let mut sock_lock = sock.lock().await;

        if !sock_lock.can_recv() {
            Timer::after_millis(10).await;
            continue;
        }
        let read_result = sock_lock.read_exact(&mut buf).await;
        core::mem::drop(sock_lock);

        debug!("Read result: {}", &read_result);

        match read_result {
            Ok(()) => {}
            Err(ReadExactError::UnexpectedEof) => {
                defmt::panic!("wut happened? the GS crashed mid-trasmission?!")
            }
            Err(
                e @ embedded_io_async::ReadExactError::Other(
                    embassy_net::tcp::Error::ConnectionReset,
                ),
            ) => {
                defmt::error!("{}", e);
                rs.signal(());
                Timer::after_millis(100).await;
                continue;
            }
        };
        publisher.publish(GsToPodMessage::read_from_buf(&buf)).await;
        Timer::after_millis(10).await;
    }
}

#[embassy_executor::task]
async fn tx_task(
    sock: &'static Mutex<NoopRawMutex, TcpSocket<'static>>,
    receiver: TxReceiver<'static>,
    rs: &'static ReconnectSignal,
    cs: &'static ConnectedSignal,
) -> ! {
    loop {
        debug!("Waiting for message");
        let msg = receiver.receive().await;
        let bytes = msg.dp.as_bytes();

        debug!("writing {:?}", bytes);

        loop {
            if rs.signaled() {
                let _ = cs.wait().await;
            }

            debug!("Actually writing");
            let mut sock_lock = sock.lock().await;
            let tx_result = sock_lock.write_all(&bytes).await;
            // let _ = sock_lock.flush().await;
            drop(sock_lock);

            debug!("Written");

            match tx_result {
                Ok(()) => {
                    break;
                }
                Err(embassy_net::tcp::Error::ConnectionReset) => {
                    rs.signal(());
                    Timer::after_millis(200).await;
                }
            }
        }
    }
}

fn get_remote_endpoint() -> (Ipv4Address, u16) {
    let (oct, port) = unsafe { config::GS_IP_ADDRESS };
    (Ipv4Address::new(oct[0], oct[1], oct[2], oct[3]), port)
}

#[embassy_executor::task]
async fn restore_connection(
    rs: &'static ReconnectSignal,
    csrx: &'static ConnectedSignal,
    cstx: &'static ConnectedSignal,
    sock: &'static Mutex<NoopRawMutex, TcpSocket<'static>>,
    stack: Stack<'static>,
) -> ! {
    static COMMS_BUFFERS: StaticCell<CommsBuffers> = StaticCell::new();

    let comms_buffers = COMMS_BUFFERS.init_with(|| CommsBuffers {
        rx: [0; RX_BUFFER_SIZE],
        tx: [0; TX_BUFFER_SIZE],
    });

    loop {
        let _ = rs.wait().await;
        // signal for the rx and tx tasks that we are trying to reconnect,
        // so that they don't try to read / write to the socket while we
        // are reconnecting.
        csrx.reset();
        cstx.reset();
        rs.signal(());

        info!("Attempting to reconnect");

        // Reconnection triggered
        let mut sock_lock = sock.lock().await;
        let _ = sock_lock.flush().await;
        sock_lock.abort();
        let _ = sock_lock.flush().await;

        // make sure to drop the previous socket before initializing the new one.
        // we are using the same buffers.
        unsafe {
            core::ptr::drop_in_place(&mut *sock_lock as *mut TcpSocket<'_>);
            let rx = core::ptr::from_mut(&mut comms_buffers.rx);
            let tx = core::ptr::from_mut(&mut comms_buffers.tx);
            core::ptr::write(
                &mut *sock_lock as *mut TcpSocket<'_>,
                TcpSocket::new(stack, &mut *rx, &mut *tx),
            );
        }

        sock_lock.set_timeout(Some(embassy_time::Duration::from_secs(10)));

        loop {
            match sock_lock.connect(get_remote_endpoint()).await {
                Ok(()) => {
                    break;
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        }

        rs.reset();
        csrx.signal(());
        cstx.signal(());
    }
}

#[embassy_executor::task]
async fn eth_task(mut runner: embassy_net::Runner<'static, EthDevice>) -> ! {
    runner.run().await
}

impl GsCommsLayerInitializable for EthernetGsCommsLayerInitializer {
    type CommsLayer = EthernetGsCommsLayer;

    async fn init(self, spawner: Spawner) -> Self::CommsLayer {
        let Self {
            seed,
            device,
            config,
        } = self;

        static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
        let (stack, runner) =
            embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);
        unwrap!(spawner.spawn(eth_task(runner)));

        info!("Waiting for config to come up");
        stack.wait_config_up().await;

        info!("Config is up");

        static COMMS_BUFFERS_INIT: StaticCell<CommsBuffers> = StaticCell::new();

        let comms_buffers = COMMS_BUFFERS_INIT.init(CommsBuffers {
            rx: [0; RX_BUFFER_SIZE],
            tx: [0; TX_BUFFER_SIZE],
        });

        let mut sock = TcpSocket::new(stack, &mut comms_buffers.rx, &mut comms_buffers.tx);
        sock.set_timeout(Some(embassy_time::Duration::from_secs(10)));

        let remote_endpoint = get_remote_endpoint();

        let sock = loop {
            let r = sock.connect(remote_endpoint).await;
            if let Err(e) = r {
                error!("{}", e);
                // hprintln!("connect error: {:?}", e);
                Timer::after_secs(1).await;
                continue;
            }

            break sock;
        };

        info!("Connected to gs");

        static SOCK: StaticCell<Mutex<NoopRawMutex, TcpSocket<'static>>> = StaticCell::new();
        let sock = SOCK.init(Mutex::new(sock));

        static COMMS_CORE: StaticCell<CommsCore> = StaticCell::new();

        let core = COMMS_CORE.init(CommsCore {
            rx_channel: RxChannel::new(),
            tx_channel: TxChannel::new(),
        });

        static RECONNECT_SIGNAL: ReconnectSignal = ReconnectSignal::new();
        static CONNECTED_SIGNAL_RX: ConnectedSignal = ConnectedSignal::new();
        static CONNECTED_SIGNAL_TX: ConnectedSignal = ConnectedSignal::new();

        unwrap!(spawner.spawn(restore_connection(
            &RECONNECT_SIGNAL,
            &CONNECTED_SIGNAL_RX,
            &CONNECTED_SIGNAL_TX,
            sock,
            stack
        )));

        let rx_publisher = unwrap!(core.rx_channel.publisher());

        unwrap!(spawner.spawn(rx_task(
            sock,
            rx_publisher,
            &RECONNECT_SIGNAL,
            &CONNECTED_SIGNAL_RX
        )));

        let tx_subscriber = core.tx_channel.receiver();

        unwrap!(spawner.spawn(tx_task(
            sock,
            tx_subscriber,
            &RECONNECT_SIGNAL,
            &CONNECTED_SIGNAL_TX
        )));

        CONNECTED_SIGNAL_TX.signal(());
        CONNECTED_SIGNAL_RX.signal(());

        fn ticks() -> u64 {
            Instant::now().as_ticks()
        }

        debug!("Handshaking");
        let tx = core.tx_channel.sender();
        tx.send(PodToGsMessage {
            // 0xE981A1EA0B1A4199
            dp: Datapoint::new(Datatype::CommandHash, COMMAND_HASH, ticks()),
        })
        .await;
        tx.send(PodToGsMessage {
            // 0xDEEDB95C8FC613FF
            dp: Datapoint::new(Datatype::EventsHash, EVENTS_HASH, ticks()),
        })
        .await;
        tx.send(PodToGsMessage {
            // 0xE1BC61029CE8A7B3
            dp: Datapoint::new(Datatype::DataHash, DATA_HASH, ticks()),
        })
        .await;
        tx.send(PodToGsMessage {
            // 0xB13F6E1D797FE777
            dp: Datapoint::new(Datatype::ConfigHash, CONFIG_HASH, ticks()),
        })
        .await;
        tx.send(PodToGsMessage {
            dp: Datapoint::new(Datatype::FrontendHeartbeating, 0, ticks()),
        })
        .await;

        EthernetGsCommsLayer { cc: core }
    }
}
