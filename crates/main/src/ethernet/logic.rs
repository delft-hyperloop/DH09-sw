//! The logic behind ethernet. Made using an FSM that checks the state of the
//! socket used for communications.

use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::State;
use embassy_net::tcp::TcpSocket;
use embassy_net::Ipv4Address;
use embassy_net::Stack;
use embassy_net::StackResources;
use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::GenericPhy;
use embassy_stm32::eth::InterruptHandler;
use embassy_stm32::eth::PacketQueue;
use embassy_stm32::interrupt;
use embassy_stm32::Peripherals;
use embassy_time::Timer;
use embedded_io_async::Read;
use embedded_io_async::ReadExactError;
use embedded_io_async::Write;
use lib::config;
use static_cell::StaticCell;

use crate::ethernet::types::Comms;
use crate::ethernet::types::EthDevice;
use crate::ethernet::types::GsToPodMessage;
use crate::ethernet::types::GsToPodPublisher;
use crate::ethernet::types::GsToPodSubscriber;
use crate::ethernet::types::PodToGsPublisher;
use crate::ethernet::types::PodToGsSubscriber;

pub struct GsMaster {
    stack: Stack<'static>,
    socket: TcpSocket<'static>,
    remote: (Ipv4Address, u16),
    pub comms: Comms,
}

impl GsMaster {
    pub async fn init(
        p: Peripherals,
        spawner: Spawner,
        irq: impl interrupt::typelevel::Binding<interrupt::typelevel::ETH, InterruptHandler>,
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
            irq,
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
        let (stack, runner) =
            embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

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
            comms: Comms::new(),
        }
    }

    pub async fn run(&mut self) -> ! {
        let rx_buffer = [0; GsToPodMessage::SIZE];
        let receiver = self.comms.tx_channel.receiver();
        let publisher = self.comms.rx_channel.publisher().unwrap();

        loop {
            let state: embassy_net::tcp::State = self.socket.state();

            match state {
                State::Closed | State::Closing | State::CloseWait => {
                    self.reconnect().await;
                }
                State::Established => {
                    self.receive(rx_buffer).await;
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

    async fn transmit(&mut self, receiver: PodToGsSubscriber<'static>) {
        let msg = receiver.receive().await;
        let bytes = msg.dp.as_bytes();

        let tx_result = self.socket.write_all(&bytes).await;

        match tx_result {
            Ok(()) => {}
            Err(embassy_net::tcp::Error::ConnectionReset) => {
                // TODO: Trigger reconnection and somehow keep the bytes that we were supposed
                //       to send so we send them again after reconnecting
                Timer::after_millis(200).await;
            }
        }
    }

    async fn receive(
        &mut self,
        mut buf: [u8; GsToPodMessage::SIZE],
        transmitter: GsToPodPublisher<'static>,
    ) {
        if !self.socket.can_recv() {
            Timer::after_millis(5).await;
        }

        let read_result = self.socket.read_exact(&mut buf).await;

        match read_result {
            Ok(()) => {}
            Err(ReadExactError::UnexpectedEof) => {
                defmt::panic!("wut happened? the GS crashed mid-transmission?!")
            }
            Err(
                e @ embedded_io_async::ReadExactError::Other(
                    embassy_net::tcp::Error::ConnectionReset,
                ),
            ) => {
                defmt::error!("{}", e);
                // TODO: trigger reconnection
                Timer::after_millis(100).await;
            }
        };

        transmitter
            .publish(GsToPodMessage::read_from_buf(&buf))
            .await;
        Timer::after_millis(5).await;
    }

    pub fn receiver(&self) -> GsToPodSubscriber {
        self.comms.rx_channel.subscriber().unwrap()
    }

    pub fn transmitter(&self) -> PodToGsPublisher {
        self.comms.tx_channel.sender()
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

// TODO: in the future, this should support multiple addresses or discover the
//       address of a groundstation
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
