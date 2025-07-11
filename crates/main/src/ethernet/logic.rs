//! The logic behind ethernet. Made using an FSM that checks the state of the
//! socket used for communications.

use core::fmt::Debug;

use cortex_m::peripheral::SCB;
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::ConnectError;
use embassy_net::tcp::State;
use embassy_net::tcp::TcpSocket;
use embassy_net::Config;
use embassy_net::Ipv4Address;
use embassy_net::Stack;
use embassy_net::StackResources;
use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::GenericPhy;
use embassy_stm32::eth::InterruptHandler;
use embassy_stm32::eth::PacketQueue;
use embassy_stm32::interrupt;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_time::Duration;
use embassy_time::Timer;
use embassy_time::WithTimeout;
use embedded_io_async::Read;
use embedded_io_async::ReadExactError;
use embedded_io_async::Write;
use lib::config;
use lib::config::Command;
use lib::config::Datatype;
use lib::config::COMMAND_HASH;
use lib::config::CONFIG_HASH;
use lib::config::DATA_HASH;
use lib::Datapoint;
use static_cell::StaticCell;

use crate::ethernet::types::ticks;
use crate::ethernet::types::EthPeripherals;
use crate::ethernet::types::GsToPodMessage;
use crate::ethernet::types::GsToPodPublisher;
use crate::ethernet::types::PodToGsMessage;
use crate::ethernet::types::PodToGsPublisher;
use crate::ethernet::types::PodToGsSubscriber;
use crate::ethernet::types::RX_BUFFER_SIZE;
use crate::ethernet::types::SOCKET_KEEP_ALIVE;
use crate::ethernet::types::TX_BUFFER_SIZE;
use crate::ethernet::EthDevice;

/// Boolean used to check if the hashes have been sent or not.
/// Shared between the `timeout_for_sending_hashes` task and the `connect`
/// method from the `GsMaster`
static HASH_TIMEOUT_FLAG: Mutex<CriticalSectionRawMutex, bool> = Mutex::new(false);

/// Struct used to communicate over ethernet with the GS.
pub struct GsMaster {
    /// The TCP stack used to create new sockets
    stack: Stack<'static>,
    /// The socket used for communicating with the ground station
    socket: TcpSocket<'static>,
    /// The IP addresses that the socket should try to connect to
    remotes: [(Ipv4Address, u16); config::IP_ADDRESS_COUNT],
    /// Receiver for the transmission channel
    tx_receiver: PodToGsSubscriber<'static>,
    /// Transmitter for the receiving channel
    rx_transmitter: GsToPodPublisher<'static>,
    /// Transmitter for the transmission channel
    tx_transmitter: PodToGsPublisher<'static>,
    /// Flag that triggers a reconnection (creates a new socket)
    should_reconnect: bool,
    /// Flag used when reconnecting to indicate if it was a faulty connection
    /// (not critical) or we should actually emergency brake (related to the bug
    /// where it
    // only sends a connection established message, but can't send or transmit
    // anything).
    connection_is_broken: bool,
}

impl Debug for GsMaster {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ep = self.socket.remote_endpoint().unwrap();
        core::write!(f, "GsMaster with socket {:?}:{:?}", ep.addr, ep.port)
    }
}

/// Buffer used by the TCP stack when receiving
static mut RX_BUFFER: [u8; RX_BUFFER_SIZE] = [0u8; RX_BUFFER_SIZE];
/// Buffer used by the TCP stack when transmitting
static mut TX_BUFFER: [u8; TX_BUFFER_SIZE] = [0u8; TX_BUFFER_SIZE];

// /// Buffer used to retransmit a message that failed to transmit because of a
// /// connection reset error
// static mut TX_BYTES: [u8; 20] = [0; 20];

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
        let config = Config::dhcpv4(Default::default());

        // static IPv4 address
        // let config = Config::ipv4_static(embassy_net::StaticConfigV4 {
        //     address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 0, 100), 24),
        //     gateway: None,
        //     dns_servers: Default::default(),
        // });

        // Get the IPv4 address of the GS
        let remotes = get_remote_endpoints();
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
        // SAFETY: see explanation under reconnect()
        #[allow(static_mut_refs)]
        let socket: TcpSocket =
            unsafe { TcpSocket::new(stack, RX_BUFFER.as_mut(), TX_BUFFER.as_mut()) };

        Self {
            stack,
            socket,
            remotes,
            tx_receiver,
            rx_transmitter,
            tx_transmitter,
            should_reconnect: false,
            connection_is_broken: false,
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
    pub async fn run_net_fsm(
        &'static mut self,
        signal_connected: &'static Signal<NoopRawMutex, bool>,
    ) -> ! {
        info!("Running the ethernet fsm");

        info!("Connecting to the GS");
        self.connect().await;
        signal_connected.signal(true);
        info!("Connected to the GS");

        loop {
            if self.should_reconnect {
                info!("Should-reconnect triggered");
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

        let mut index: usize = 0;
        loop {
            // Try to connect to a different IP address every time the socket can't reach
            // the server
            let remote = self.remotes[index];
            debug!("Trying to connect to {:?}", remote);

            match self.socket.connect(remote).await {
                Ok(()) => {
                    debug!(
                        "socket connected, state={}, endpoint={}",
                        self.socket.state(),
                        self.socket.remote_endpoint()
                    );
                    break;
                }
                Err(ConnectError::InvalidState) => {
                    error!("Connect Error Invalid State (already connected)");
                    break;
                }
                Err(e) => {
                    debug!(
                        "Connect error (probably waiting for the GS server to start) {}",
                        e
                    );
                    index = (index + 1) % config::IP_ADDRESS_COUNT;
                }
            }
        }

        debug!("handshaking (sending hashes)");

        // Sends the hash messages to the ground station. If the first one doesn't get
        // sent in 200 milliseconds, it triggers a reconnection (related to the bug
        // where it only sends a connection established message, but can't send
        // or transmit anything)
        match self
            .tx_transmitter
            .send(PodToGsMessage {
                dp: Datapoint::new(Datatype::CommandHash, COMMAND_HASH, ticks()),
            })
            .with_timeout(Duration::from_millis(200))
            .await
        {
            Ok(_) => {
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
                info!("connected, endpoint={:?}", self.socket.remote_endpoint());
            }
            Err(e) => {
                warn!("Timeout for sending hashes has expired with error {:?}! Triggering a reconnection!", e);
                self.should_reconnect = true;
                self.connection_is_broken = true;
            }
        }

        // let mut mutex_lock = HASH_TIMEOUT_FLAG.lock().await;
        // *mutex_lock = true;
        // core::mem::drop(mutex_lock);
    }

    /// Reconnects to the GS if the connection drops by creating a new socket.
    async fn reconnect(&mut self) {
        info!("Reconnecting to the GS");

        // If the connection is broken, don't trigger an emergency and attempt to
        // recover.
        if !self.connection_is_broken {
            self.rx_transmitter
                .publish(GsToPodMessage {
                    command: Command::ReconnectEmergency(0),
                })
                .await;
        } else {
            self.connection_is_broken = false;
        }

        // flush whatever was still written to the socket
        if let Err(e) = self.socket.flush().await {
            defmt::error!("could not flush socket: {:?}", e);
            #[cfg(debug_assertions)]
            defmt::assert!(false, "could not flush socket: {}/{:?}", e, e);
        }

        // without an allocator (feature="alloc") smoltcp can only hold 1 active socket
        // at a time. In order to create a brand new connection, we also need a
        // socket, which requires dropping the previous one in order to
        // accomodate for it.
        #[allow(static_mut_refs)]
        // SAFETY: replace the socket in memory with a new socket.
        unsafe {
            // TcpSocket::new takes the &mut [u8] that we give (rx and tx buffers) and
            // transmutes it to a &'static mut [u8].
            // The destructor (impl Drop) for TcpSocket removes the socket from the
            // SocketSet of the stack. This in turn drops the &'static mut [u8]
            // reference that pointed to the buffers. Thus, a new &mut [u8] to the same
            // memory is safe, since we know the last one is now gone.
            core::ptr::drop_in_place(&mut self.socket as *mut TcpSocket<'_>);
            RX_BUFFER.fill(0);
            TX_BUFFER.fill(0);
            core::ptr::write(
                &mut self.socket as *mut TcpSocket<'_>,
                TcpSocket::new(self.stack, RX_BUFFER.as_mut(), TX_BUFFER.as_mut()),
            );
        }

        // Connect to the GS
        self.connect().await;
        self.should_reconnect = false;
    }

    /// Transmits the messages from the PodToGsChannel.
    async fn transmit(&mut self) {
        let msg = self.tx_receiver.receive().await;

        let buf = msg.dp.as_bytes();

        let tx_result = self.socket.write_all(&buf).await;

        match tx_result {
            Ok(()) => {}
            Err(embassy_net::tcp::Error::ConnectionReset) => {
                self.should_reconnect = true;
            }
        }
    }

    /// Receives messages over ethernet and publishes them to the
    /// GsToPodChannel. If this fails, trigger a reconnection.
    async fn receive(&mut self) {
        // Buffer should have the size of a message so it can only store one message.
        let mut buf = [0; GsToPodMessage::SIZE];

        if !self.socket.can_recv() {
            return;
        }

        // Reads and stores in the buffer an amount of bytes equal to the size of the
        // buffer.
        let read_result = self.socket.read_exact(&mut buf).await;

        trace!("reading from tcp socket: {}", &read_result);

        match read_result {
            Ok(()) => {}
            Err(ReadExactError::UnexpectedEof) => {
                // we have never encountered this
                #[cfg(debug_assertions)]
                defmt::panic!("wut happened? the GS crashed mid-transmission?!");
                #[cfg(not(debug_assertions))]
                defmt::error!("tcp read error: UnexpectedEof");
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

/// get ground station [`Ipv4Address`]
fn get_remote_endpoints() -> [(Ipv4Address, u16); config::IP_ADDRESS_COUNT] {
    let ips = config::GS_IP_ADDRESSES;
    ips.map(|x| (Ipv4Address::new(x.0[0], x.0[1], x.0[2], x.0[3]), x.1))
}

/// Task that runs the network stack
#[embassy_executor::task]
async fn eth_task(mut runner: embassy_net::Runner<'static, EthDevice>) -> ! {
    info!("Running the TCP stack");
    runner.run().await
}

/// Task that triggers a hardware reset 1 second after it gets spawned.
#[allow(dead_code)]
#[embassy_executor::task]
async fn hardware_reset_timeout() {
    info!("Starting watchdog for hashes");

    Timer::after_secs(1).await;
    let mut mutex_lock = HASH_TIMEOUT_FLAG.lock().await;
    if !*mutex_lock {
        SCB::sys_reset()
    }
    // how does this ever run if you've reset??
    *mutex_lock = false;
}
