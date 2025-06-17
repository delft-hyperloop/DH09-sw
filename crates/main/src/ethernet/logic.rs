//! The logic behind ethernet. Made using an FSM that checks the state of the socket used for communications.

use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Ipv4Address, Stack, StackResources};
use embassy_net::tcp::State;
use embassy_net::tcp::TcpSocket;
use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::GenericPhy;
use embassy_stm32::eth::PacketQueue;
use embassy_stm32::Peripherals;
use static_cell::StaticCell;
use lib::config;
use crate::ethernet::types::{EthDevice, GsToPodPublisher};
use crate::ethernet::types::PodToGsSubscriber;

pub struct GsMaster {
    stack: Stack<'static>,
    socket: TcpSocket<'static>,
    remote: (Ipv4Address, u16),
    pod_to_gs_subscriber: PodToGsSubscriber<'static>,
    gs_to_pod_publisher: GsToPodPublisher<'static>,
}

impl GsMaster {
    pub async fn init(
        p: Peripherals,
        pod_to_gs_subscriber: PodToGsSubscriber<'static>,
        gs_to_pod_publisher: GsToPodPublisher<'static>,
        spawner: Spawner,
    ) -> Self {
        
        // Get the mac address of the pod
        let mac_addr = lib::config::POD_MAC_ADDRESS;
        // Get an IPv4 address for the pod
        let config = embassy_net::Config::dhcpv4(Default::default());
        // Get the IPv4 address of the GS
        let remote = get_remote_endpoint();
        // Random seed
        let seed: u64 = 0x1234567890ABCDEF;
        
        static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
        // warning: Not all STM32H7 devices have the exact same pins here
        // for STM32H747XIH, replace p.PB13 for PG12
        let device = Ethernet::new(
            PACKETS.init(PacketQueue::<4, 4>::new()),
            p.ETH,
            Irqs,
            p.PA1, // ref_clk
            p.PA2, // mdio
            p.PC1, // eth_mdc
            p.PA7, // CRS_DV: Carrier Sense
            p.PC4, // RX_D0: Received Bit 0
            p.PC5, // RX_D1: Received Bit 1
            //choose one:
            p.PB12, // FOR MPCB (TX_D0: Transmit Bit 0)
            // p.PG13, // FOR NUCLEO (TX_D0: Transmit Bit 0)
            p.PB13, // TX_D1: Transmit Bit 1
            //choose one:
            p.PB11, //FOR MPCB (TX_EN: Transmit Enable)
            // p.PG11, // FOR NUCLEO (TX_EN: Transmit Enable)
            GenericPhy::new(0),
            mac_addr,
        );
        
        // Resources for the TCP stack
        static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
        
        // Initialize the TCP stack and its runner 
        let (stack, runner) = embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);
        
        info!("Waiting for ethernet peripheral to be configured");
        stack.wait_config_up().await;
        info!("Ethernet peripheral configured");
        
        // Spawn the task that runs the TCP stack
        unwrap!(spawner.spawn(eth_task(runner)));
        
        // Create a new socket for the connection
        let socket: TcpSocket = TcpSocket::new(stack, rx_buffer, tx_buffer);

        Self { 
            stack,
            socket,
            remote,
            pod_to_gs_subscriber,
            gs_to_pod_publisher
        }
    }

    pub async fn run(&mut self) -> ! {
        loop {
            let state: embassy_net::tcp::State = self.socket.state();

            match state {
                State::Closed | State::Closing | State::CloseWait => {
                    self.reconnect().await;
                }
                State::Established => {
                    self.receive().await;
                    self.transmit().await;
                }
                // and other states, I haven't looked into them
                _ => {}
            }
        }
    }

    async fn connect(&mut self) {
        info!("Connecting to the GS");
        loop {
            match self.socket.connect(self.remote).await {
                Ok(()) => break,
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }

    async fn reconnect(&mut self) {
        info!("Reconnecting to the GS");
        
        // flush whatever was still written to the socket
        let _ = self.socket.flush();
        // close the socket
        self.socket.abort();
        
        // drop the socket??
        
        
        // initialize a new one
        self.socket = TcpSocket::new(self.stack, rx_buffer, tx_buffer);
        
        // Connect to the GS
        self.connect().await;
    }

    async fn transmit(&mut self) {
        todo!()
    }

    async fn receive(&mut self) {
        todo!()
    }
}

// Closed,
// Listen,
// SynSent,
// SynReceived,
// Established,
// FinWait1,
// FinWait2,
// CloseWait,
// Closing,
// LastAck,
// TimeWait,

// TODO: in the future, this should support multiple addresses or discover the address of a groundstation
/// get ground station [`Ipv4Address`]
fn get_remote_endpoint() -> (Ipv4Address, u16) {
    // SAFETY: read-only static defined at compile time
    let (oct, port) = unsafe { config::GS_IP_ADDRESS };
    (Ipv4Address::new(oct[0], oct[1], oct[2], oct[3]), port)
}

/// Task that runs the network stack
#[embassy_executor::task]
async fn eth_task(mut runner: embassy_net::Runner<'static, EthDevice>) -> ! {
    info!("Running the TCP stack");
    runner.run().await
}
