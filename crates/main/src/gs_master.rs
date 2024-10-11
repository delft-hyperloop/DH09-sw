use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_stm32::{
    eth::{generic_smi::GenericSMI, Ethernet},
    peripherals::ETH,
};
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
    pub async fn new<I>(comms: I, spawner: Spawner) -> &'static GsMaster<I>
    where
        I: GsCommsLayerInitializable<CommsLayer = GsCommsLayerImpl>,
    {
        let comms = comms.init(spawner).await;
        static GS_MASTER: StaticCell<GsMaster<GsCommsLayerImpl>> = StaticCell::new();
        let gs_master = GS_MASTER.init(GsMaster { comms });
        gs_master
    }
}

struct GsToPodMessage {}
struct PodToGsMessage {}

pub trait GsCommsLayer {
    async fn send(&self, data: &PodToGsMessage);
    async fn receive(&self) -> GsToPodMessage;
}

pub trait GsCommsLayerInitializable {
    type CommsLayer: GsCommsLayer;

    async fn init(self, spawner: Spawner) -> Self::CommsLayer;
}

pub struct EthernetGsCommsLayer {}

impl GsCommsLayer for EthernetGsCommsLayer {
    async fn send(&mut self, _data: &PodToGsMessage) {
        todo!()
    }

    async fn receive(&mut self) -> GsToPodMessage {
        todo!()
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
        EthernetGsCommsLayer {}
    }
}
