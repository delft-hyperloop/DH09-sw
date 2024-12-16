use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{
    tcp::{TcpReader, TcpSocket},
    Ipv4Address, StackResources,
};
use embassy_stm32::{
    eth::{generic_smi::GenericSMI, Ethernet},
    peripherals::ETH,
};
use embassy_sync::{
    blocking_mutex::raw::NoopRawMutex,
    channel::Channel,
    pubsub::{PubSubChannel, Publisher, Subscriber},
};
use embassy_time::Timer;
use embedded_io_async::Read;
use static_cell::StaticCell;

type GsCommsLayerImpl = EthernetGsCommsLayer;

/// Main task responsible for communicating with the ground station.
#[embassy_executor::task]
async fn gs_main(master: GsMaster<GsCommsLayerImpl>) -> ! {
    loop {}
}

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

#[derive(Clone, Debug)]
struct GsToPodMessage {}

impl GsToPodMessage {
    const SIZE: usize = 8;

    pub fn read_from_buf(buf: &[u8; Self::SIZE]) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
struct PodToGsMessage {}

pub trait GsCommsLayer {
    fn subscribe(&self) -> RxSubscriber<'_>;
    async fn send(&self, message: PodToGsMessage);
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

    async fn send(&self, message: PodToGsMessage) {
        self.cc.tx_channel.send(message).await;
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
type TxSender<'a> = embassy_sync::channel::Sender<'a, NoopRawMutex, PodToGsMessage, TX_CAP>;

const RX_CAP: usize = 8;
const RX_SUBS: usize = 4;
const RX_PUBS: usize = 1;
type RxChannel = PubSubChannel<NoopRawMutex, GsToPodMessage, RX_CAP, RX_SUBS, RX_PUBS>;
type RxPublisher<'a> = Publisher<'a, NoopRawMutex, GsToPodMessage, RX_CAP, RX_SUBS, RX_PUBS>;
pub type RxSubscriber<'a> = Subscriber<'a, NoopRawMutex, GsToPodMessage, RX_CAP, RX_SUBS, RX_PUBS>;

#[embassy_executor::task]
async fn rx_task(mut rx: TcpReader<'static>, publisher: RxPublisher<'static>) -> ! {
    let mut buf = [0; GsToPodMessage::SIZE];

    loop {
        unwrap!(rx.read_exact(&mut buf).await);
        publisher.publish(GsToPodMessage::read_from_buf(&buf)).await;
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

        static COMMS_BUFFERS: StaticCell<CommsBuffers> = StaticCell::new();

        let CommsBuffers { rx, tx } = COMMS_BUFFERS.init(CommsBuffers {
            rx: [0; RX_BUFFER_SIZE],
            tx: [0; TX_BUFFER_SIZE],
        });

        let mut sock = TcpSocket::new(stack, rx, tx);
        sock.set_timeout(Some(embassy_time::Duration::from_secs(10)));

        let remote_endpoint = (Ipv4Address::new(192, 168, 1, 17), 8000);

        let mut sock = loop {
            let r = sock.connect(remote_endpoint).await;
            if let Err(e) = r {
                error!("{}", e);
                // hprintln!("connect error: {:?}", e);
                Timer::after_secs(1).await;
                continue;
            }

            break sock;
        };

        static SOCK: StaticCell<TcpSocket<'static>> = StaticCell::new();
        let sock = SOCK.init(sock);

        let (rx, tx) = sock.split();

        static COMMS_CORE: StaticCell<CommsCore> = StaticCell::new();

        let core = COMMS_CORE.init(CommsCore {
            rx_channel: RxChannel::new(),
            tx_channel: TxChannel::new(),
        });

        let rx_publisher = unwrap!(core.rx_channel.publisher());

        unwrap!(spawner.spawn(rx_task(rx, rx_publisher)));

        EthernetGsCommsLayer { cc: core }
    }
}
