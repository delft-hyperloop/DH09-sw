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
use embassy_time::Instant;
use embassy_time::Timer;
use embedded_io_async::Read;
use embedded_io_async::ReadExactError;
use embedded_io_async::Write;
use lib::config;
use lib::config::Datatype;
use lib::config::COMMAND_HASH;
use lib::config::CONFIG_HASH;
use lib::config::DATA_HASH;
use lib::Datapoint;
use static_cell::StaticCell;

use crate::ethernet::types::GsComms;
use crate::ethernet::types::EthDevice;
use crate::ethernet::types::GsToPodMessage;
use crate::ethernet::types::GsToPodPublisher;
use crate::ethernet::types::GsToPodSubscriber;
use crate::ethernet::types::PodToGsMessage;
use crate::ethernet::types::PodToGsPublisher;
use crate::ethernet::types::PodToGsSubscriber;
use crate::ethernet::types::RX_BUFFER_SIZE;
use crate::ethernet::types::TX_BUFFER_SIZE;

pub struct GsMaster {
    stack: Stack<'static>,
    socket: TcpSocket<'static>,
    remote: (Ipv4Address, u16),
    pub comms: GsComms,
    should_reconnect: bool,
}

/// Buffers used by the TCP stack when transmitting and receiving
static mut RX_BUFFER: [u8; RX_BUFFER_SIZE] = [0; RX_BUFFER_SIZE];
static mut TX_BUFFER: [u8; TX_BUFFER_SIZE] = [0; TX_BUFFER_SIZE];

impl GsMaster {
    /// Initializes the TCP stack and spawns a task for its runner.
    ///
    /// # Returns:
    /// - An instance of the `GsMaster`struct used to communicate over ethernet.
    pub async fn init(
        p: Peripherals,
        spawner: Spawner,
        irq: impl interrupt::typelevel::Binding<interrupt::typelevel::ETH, InterruptHandler> + 'static,
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

        // Create a new socket for the connection. Pass static mutable references to the
        // buffers that should be used for transmitting and receiving.
        let socket: TcpSocket = unsafe { TcpSocket::new(stack, &mut RX_BUFFER, &mut TX_BUFFER) };

        Self {
            stack,
            socket,
            remote,
            comms: GsComms::new(),
            should_reconnect: false,
        }
    }

    /// Runs the FSM for the GsMaster. Uses the state of the socket to determine
    /// what it should do. Also checks the internal reconnection flag if it
    /// needs to reconnect or not.
    ///
    /// # States:
    /// - `Reconnect`: if socket is in one of the following states: Closed,
    ///   Closing, CloseWait...??? It attempts to reconnect by calling the
    ///   `reconnect` method.
    /// - `Connected`: if socket is in state Established. It sends and transmits
    ///   data.
    /// - `First Connection`: if socket is in state Established for the first
    ///   time. Attempts to connect and sends the hashes to the GS.
    ///
    /// Closed,
    /// Listen,
    /// SynSent,
    /// SynReceived,
    /// Established,
    /// FinWait1,
    /// FinWait2,
    /// CloseWait,
    /// Closing,
    /// LastAck,
    /// TimeWait
    pub async fn run(&mut self) -> ! {
        let receiver = self.comms.tx_channel.receiver();
        let publisher = self.comms.rx_channel.publisher().unwrap();
        let mut first_connection: bool = true;

        loop {
            if self.should_reconnect {
                self.reconnect().await;
            }
            let state = self.socket.state();

            match state {
                State::Closed | State::Closing | State::CloseWait => {
                    self.reconnect().await;
                }
                State::Established if first_connection => {
                    first_connection = false;
                    self.connect(true).await;
                }
                State::Established => {
                    self.receive(publisher).await;
                    self.transmit(receiver).await;
                }
                // and other states, I haven't looked into them
                _ => {}
            }
        }
    }

    /// Makes the initial connection with the GS. If successful, it sends the
    /// hashes for the config, commands, and data files to the GS.
    async fn connect(&mut self, send_hashes: bool) {
        info!("Connecting to the GS");
        loop {
            match self.socket.connect(self.remote).await {
                Ok(()) => break,
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
        fn ticks() -> u64 {
            Instant::now().as_ticks()
        }

        if send_hashes {
            debug!("Handshaking");
            self.comms
                .tx_channel
                .send(PodToGsMessage {
                    dp: Datapoint::new(Datatype::CommandHash, COMMAND_HASH, ticks()),
                })
                .await;
            self.comms
                .tx_channel
                .send(PodToGsMessage {
                    dp: Datapoint::new(Datatype::DataHash, DATA_HASH, ticks()),
                })
                .await;
            self.comms
                .tx_channel
                .send(PodToGsMessage {
                    dp: Datapoint::new(Datatype::ConfigHash, CONFIG_HASH, ticks()),
                })
                .await;
            self.comms
                .tx_channel
                .send(PodToGsMessage {
                    dp: Datapoint::new(Datatype::FrontendHeartbeating, 0, ticks()),
                })
                .await;
        }
    }

    async fn reconnect(&mut self) {
        info!("Reconnecting to the GS");

        // flush whatever was still written to the socket
        let _ = self.socket.flush();
        // close the socket
        self.socket.abort();

        // drop the socket??

        // initialize a new one with the same buffers
        self.socket = unsafe { TcpSocket::new(self.stack, &mut RX_BUFFER, &mut TX_BUFFER) };

        // Connect to the GS
        self.connect(false).await;
        self.should_reconnect = false;
    }

    async fn transmit(&mut self, receiver: PodToGsSubscriber<'static>) {
        let msg = receiver.receive().await;
        let bytes = msg.dp.as_bytes();

        let tx_result = self.socket.write_all(&bytes).await;

        match tx_result {
            Ok(()) => {}
            Err(embassy_net::tcp::Error::ConnectionReset) => {
                self.should_reconnect = true;
                // TODO: Trigger reconnection and somehow keep the bytes that we were supposed
                //       to send so we send them again after reconnecting
                Timer::after_millis(200).await;
            }
        }
    }

    async fn receive(&mut self, transmitter: GsToPodPublisher<'static>) {
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
                self.should_reconnect = true;
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
