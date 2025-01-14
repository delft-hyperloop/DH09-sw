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
//! The [`GsToPodMessage`] contains an FSM [`fsm::commons::data::Event`],
//! which may be [`fsm::commons::data::Event::NoEvent`] in case it
//! shouldn't be processed. In other cases, it is forwarded to the FSM.
//!
//! The [`PodToGsMessage`] may contain some very basic logging information,
//! as most of the logging happens through the logging PCB in order to let the
//! main PCB run the FSM.

use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpReader;
use embassy_net::tcp::TcpSocket;
use embassy_net::tcp::TcpWriter;
use embassy_net::Ipv4Address;
use embassy_net::Stack;
use embassy_net::StackResources;
use embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32::eth::Ethernet;
use embassy_stm32::peripherals::ETH;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::PubSubChannel;
use embassy_sync::pubsub::Publisher;
use embassy_sync::pubsub::Subscriber;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use embedded_io_async::Read;
use embedded_io_async::ReadExactError;
use embedded_io_async::Write;
use static_cell::StaticCell;

type GsCommsLayerImpl = EthernetGsCommsLayer;

pub struct GsMaster<C: GsCommsLayer> {
    comms: C,
}

impl GsMaster<GsCommsLayerImpl> {
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
    pub fn subscribe(&self) -> RxSubscriber<'_> {
        self.comms.subscribe()
    }

    pub fn transmitter(&self) -> TxSender<'_> {
        self.comms.transmitter()
    }
}

#[derive(Clone, Debug)]
pub struct GsToPodMessage {
    pub fsm_event: fsm::commons::Event,
}

impl GsToPodMessage {
    const SIZE: usize = 8;

    pub fn read_from_buf(buf: &[u8; Self::SIZE]) -> Self {
        let fsm_event = unwrap!(fsm::commons::Event::read_from_buf([buf[0], buf[1]]));

        Self { fsm_event }
    }
}

#[derive(Clone, Debug)]
pub struct PodToGsMessage {}

pub trait GsCommsLayer {
    fn subscribe(&self) -> RxSubscriber<'_>;
    fn transmitter(&self) -> TxSender<'_>;
}

pub trait GsCommsLayerInitializable {
    type CommsLayer: GsCommsLayer;

    async fn init(self, spawner: Spawner) -> Self::CommsLayer;
}

const RX_BUFFER_SIZE: usize = 8192;
const TX_BUFFER_SIZE: usize = 8192;

struct CommsBuffers {
    rx: [u8; RX_BUFFER_SIZE],
    tx: [u8; TX_BUFFER_SIZE],
}

struct CommsCore {
    rx_channel: RxChannel,
    tx_channel: TxChannel,
}

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

type EthDevice = Ethernet<'static, ETH, GenericSMI>;

pub struct EthernetGsCommsLayerInitializer {
    seed: u64,
    device: EthDevice,
    config: embassy_net::Config,
}

impl EthernetGsCommsLayerInitializer {
    pub fn new(device: EthDevice, config: embassy_net::Config) -> Self {
        Self {
            seed: 0x1234567890ABCDEF,
            device,
            config,
        }
    }
}

const TX_CAP: usize = 4;
type TxChannel = Channel<NoopRawMutex, PodToGsMessage, TX_CAP>;
type TxReceiver<'a> = embassy_sync::channel::Receiver<'a, NoopRawMutex, PodToGsMessage, TX_CAP>;
pub type TxSender<'a> = embassy_sync::channel::Sender<'a, NoopRawMutex, PodToGsMessage, TX_CAP>;

const RX_CAP: usize = 8;
const RX_SUBS: usize = 4;
const RX_PUBS: usize = 1;
type RxChannel = PubSubChannel<NoopRawMutex, GsToPodMessage, RX_CAP, RX_SUBS, RX_PUBS>;
type RxPublisher<'a> = Publisher<'a, NoopRawMutex, GsToPodMessage, RX_CAP, RX_SUBS, RX_PUBS>;
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
        let msg = receiver.receive().await;
        // TODO: convert message to bytes
        let bytes: [u8; 6] = [b'H', b'y', b't', b'e', b's', b'\n'];

        loop {
            if rs.signaled() {
                let _ = cs.wait().await;
            }

            let mut sock_lock = sock.lock().await;
            let tx_result = sock_lock.write_all(&bytes).await;
            drop(sock_lock);

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
    (Ipv4Address::new(192, 168, 1, 17), 8000)
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
        stack.wait_config_up().await;

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

        unwrap!(spawner.spawn(restore_connection(&RECONNECT_SIGNAL, &CONNECTED_SIGNAL_RX, &CONNECTED_SIGNAL_TX, sock, stack)));

        let rx_publisher = unwrap!(core.rx_channel.publisher());

        unwrap!(spawner.spawn(rx_task(sock, rx_publisher, &RECONNECT_SIGNAL, &CONNECTED_SIGNAL_RX)));

        let tx_subscriber = core.tx_channel.receiver();

        unwrap!(spawner.spawn(tx_task(sock, tx_subscriber, &RECONNECT_SIGNAL, &CONNECTED_SIGNAL_TX)));

        CONNECTED_SIGNAL_TX.signal(());
        CONNECTED_SIGNAL_RX.signal(());

        EthernetGsCommsLayer { cc: core }
    }
}
