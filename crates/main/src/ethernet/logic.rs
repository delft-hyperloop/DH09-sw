//! The logic behind ethernet. Made using an FSM that checks the state of the
//! socket used for communications.

use core::ops::Rem;
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::ConnectError;
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
use embassy_time::{Duration, Instant};
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

use crate::ethernet::types::{EthDevice, SOCKET_KEEP_ALIVE,};
use crate::ethernet::types::EthPeripherals;
use crate::ethernet::types::GsToPodMessage;
use crate::ethernet::types::GsToPodPublisher;
use crate::ethernet::types::PodToGsMessage;
use crate::ethernet::types::PodToGsPublisher;
use crate::ethernet::types::PodToGsSubscriber;
use crate::ethernet::types::RX_BUFFER_SIZE;
use crate::ethernet::types::TX_BUFFER_SIZE;

/// Struct used to communicate over ethernet with the GS.
pub struct GsMaster {
    stack: Stack<'static>,
    socket: TcpSocket<'static>,
    remote: (Ipv4Address, u16),
    tx_receiver: PodToGsSubscriber<'static>,
    rx_transmitter: GsToPodPublisher<'static>,
    tx_transmitter: PodToGsPublisher<'static>,
    should_reconnect: bool,
    tx_failed: bool,
}

/// Buffer used by the TCP stack when receiving
static mut RX_BUFFER: [u8; RX_BUFFER_SIZE] = [0; RX_BUFFER_SIZE];
/// Buffer used by the TCP stack when transmitting
static mut TX_BUFFER: [u8; TX_BUFFER_SIZE] = [0; TX_BUFFER_SIZE];

/// Buffer used to retransmit a message that failed to transmit because of a
/// connection reset error
static mut TX_BYTES: [u8; 20] = [0; 20];

impl GsMaster {
    /// Initializes the TCP stack and spawns a task for its runner.
    ///
    /// # Returns:
    /// - An instance of the `GsMaster`struct used to communicate over ethernet.
    pub async fn init(
        p: EthPeripherals,
        spawner: Spawner,
        irq: impl interrupt::typelevel::Binding<interrupt::typelevel::ETH, InterruptHandler> + 'static,
        tx_receiver: PodToGsSubscriber<'static>,
        rx_transmitter: GsToPodPublisher<'static>,
        tx_transmitter: PodToGsPublisher<'static>,
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
            p.eth,
            irq,
            p.pa1, // ref_clk
            p.pa2, // mdio
            p.pc1, // eth_mdc
            p.pa7, // CRS_DV: Carrier Sense
            p.pc4, // RX_D0: Received Bit 0
            p.pc5, // RX_D1: Received Bit 1
            //choose one:
            p.pb12, // FOR MPCB (TX_D0: Transmit Bit 0)
            // p.PG13, // FOR NUCLEO (TX_D0: Transmit Bit 0)
            p.pb13, // TX_D1: Transmit Bit 1
            //choose one:
            p.pb11, //FOR MPCB (TX_EN: Transmit Enable)
            // p.PG11, // FOR NUCLEO (TX_EN: Transmit Enable)
            GenericPhy::new(0),
            mac_addr,
        );

        // Resources for the TCP stack
        static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

        // Initialize the TCP stack and its runner
        let (stack, runner) =
            embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

        // Spawn the task that runs the TCP stack
        unwrap!(spawner.spawn(eth_task(runner)));

        info!("Waiting for ethernet peripheral to be configured");
        stack.wait_config_up().await;
        info!("Ethernet peripheral configured");

        // Create a new socket for the connection. Pass static mutable references to the
        // buffers that should be used for transmitting and receiving.
        let socket: TcpSocket = unsafe { TcpSocket::new(stack, &mut RX_BUFFER, &mut TX_BUFFER) };

        Self {
            stack,
            socket,
            remote,
            tx_receiver,
            rx_transmitter,
            tx_transmitter,
            should_reconnect: false,
            tx_failed: false,
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
    /// # Possible Socket States:
    /// - `LISTEN` represents waiting for a connection request from any remote
    ///   TCP and port.
    /// - `SYN-SENT` represents waiting for a matching connection request after
    ///   having sent a connection request.
    /// - `SYN-RECEIVED` represents waiting for a confirming connection request
    ///   acknowledgment after having both received and sent a connection
    ///   request.
    /// - `ESTABLISHED` represents an open connection, data received can be
    ///   delivered to the user. The normal state for the data transfer phase of
    ///   the connection.
    /// - `FIN-WAIT-1` represents waiting for a connection termination request
    ///   from the remote TCP, or an acknowledgment of the connection
    ///   termination request previously sent.
    /// - `FIN-WAIT-2` represents waiting for a connection termination request
    ///   from the remote TCP.
    /// - `CLOSE-WAIT` represents waiting for a connection termination request
    ///   from the local user.
    /// - `CLOSING` represents waiting for a connection termination request
    ///   acknowledgment from the remote TCP.
    /// - `LAST-ACK` represents waiting for an acknowledgment of the connection
    ///   termination request previously sent to the remote TCP (which includes
    ///   an acknowledgment of its connection termination request).
    /// - `TIME-WAIT` represents waiting for enough time to pass to be sure the
    ///   remote TCP received the acknowledgment of its connection termination
    ///   request.
    /// - `CLOSED` represents no connection state at all
    pub async fn run(&mut self) -> ! {
        info!("Running the ethernet fsm");

        info!("Connecting to the GS");
        self.connect().await;
        info!("Connected to the GS");

        loop {
            if self.should_reconnect {
                info!("Should reconnect triggered");
                self.reconnect().await;
            }
            let state = self.socket.state();

            match state {
                State::Closed
                | State::Closing
                | State::CloseWait
                | State::FinWait1
                | State::FinWait2
                | State::LastAck
                | State::TimeWait => {
                    info!("Reconnection triggered with socket state {}", state);

                    self.reconnect().await;
                }
                State::Established => {
                    self.receive().await;
                    self.transmit().await;
                }
                // If in states `Listen`, `SynSent`, or `SynReceived`, it's waiting for a connection
                // which should not be handled here.
                _ => {}
            }
        }
    }

    /// Makes the initial connection with the GS. If successful, it sends the
    /// hashes for the config, commands, and data files to the GS.
    async fn connect(&mut self) {
        // configure socket
        self.socket.set_timeout(Some(SOCKET_KEEP_ALIVE * 2));
        self.socket.set_keep_alive(Some(SOCKET_KEEP_ALIVE));
        loop {
            let mut counter = 0usize;
            match self.socket.connect(self.remote).await {
                Ok(()) => {
                    debug!("socket connected, state={}, endpoint={}", self.socket.state(), self.socket.remote_endpoint());
                    break;
                },
                Err(ConnectError::InvalidState) => {
                    error!("Connect Error Invalid State (already connected)");
                    break;
                }
                Err(e) => {
                    debug!("Connect error (probably waiting for the GS server to start): {}", e);
                    // Don't remove this timer! 
                    counter = counter.wrapping_add(1);
                    if counter.rem(200) == 0 {
                        warn!("trying to connect. state={}", self.socket.state());
                    }
                    Timer::after_millis(5).await;
                }
            }
        }
        fn ticks() -> u64 {
            Instant::now().as_ticks()
        }

        debug!("handshaking (sending hashes)");
        self.tx_transmitter
            .send(PodToGsMessage {
                dp: Datapoint::new(Datatype::CommandHash, COMMAND_HASH, ticks()),
            })
            .await;
        self.tx_transmitter
            .send(PodToGsMessage {
                dp: Datapoint::new(Datatype::DataHash, DATA_HASH, ticks()),
            })
            .await;
        self.tx_transmitter
            .send(PodToGsMessage {
                dp: Datapoint::new(Datatype::ConfigHash, CONFIG_HASH, ticks()),
            })
            .await;
        self.tx_transmitter
            .send(PodToGsMessage {
                dp: Datapoint::new(Datatype::FrontendHeartbeating, 0, ticks()),
            })
            .await;
        while self.socket.state() == State::Closed {
            warn!("waiting for network stack state to update. state={}", self.socket.state());
            Timer::after_millis(5).await;
        }
        info!("connected, endpoint={:?}", self.socket.remote_endpoint())
    }

    /// Reconnects to the GS if the connection drops by creating a new socket.
    async fn reconnect(&mut self) {
        info!("Reconnecting to the GS");

        // flush whatever was still written to the socket
        let _ = self.socket.flush();
        // close the socket
        self.socket.abort();

        // SAFETY: replace the socket in memory with a new socket.
        unsafe {
            core::ptr::drop_in_place(&mut self.socket as *mut TcpSocket<'_>);
            core::ptr::write(
                &mut self.socket as *mut TcpSocket<'_>,
                TcpSocket::new(self.stack, &mut RX_BUFFER, &mut TX_BUFFER),
            );
        }

        // Connect to the GS
        self.connect().await;
        self.should_reconnect = false;
    }

    /// Transmits the messages from the PodToGsChannel. If a transmission fails,
    /// it saves the bytes it was supposed to send in `TX_BYTES` and reattempts
    /// to send them in the next call.
    async fn transmit(&mut self) {
        if self.tx_failed {
            let mut buf: [u8; 20] = [0; 20];
            unsafe {
                buf.copy_from_slice(&TX_BYTES);
            }
            let tx_result = self.socket.write_all(&buf).await;

            match tx_result {
                Ok(()) => {
                    self.tx_failed = false;
                }
                Err(embassy_net::tcp::Error::ConnectionReset) => {
                    self.should_reconnect = true;
                }
            }

            return;
        }

        let msg = self.tx_receiver.receive().await;

        let buf = msg.dp.as_bytes();
        unsafe {
            // Copy buf into TX_BYTES
            TX_BYTES.copy_from_slice(&buf);
        }

        let tx_result = self.socket.write_all(&buf).await;

        match tx_result {
            Ok(()) => {
                self.tx_failed = false;
            }
            Err(embassy_net::tcp::Error::ConnectionReset) => {
                self.should_reconnect = true;
                self.tx_failed = true;
            }
        }
    }

    /// Receives messages over ethernet and publishes them to the
    /// GsToPodChannel. If this fails, trigger a reconnection.
    async fn receive(&mut self) {
        // Buffer should have the size of a message so it can only store one message.
        let mut buf = [0; GsToPodMessage::SIZE];

        if !self.socket.can_recv() {
            // Timer::after_millis(5).await;
            return;
        }

        // Reads and stores in the buffer an amount of bytes equal to the size of the
        // buffer.
        let read_result = self.socket.read_exact(&mut buf).await;

        trace!("reading from tcp socket: {}", &read_result);

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

        self.rx_transmitter
            .publish(GsToPodMessage::read_from_buf(&buf))
            .await;
        Timer::after_millis(5).await;
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
