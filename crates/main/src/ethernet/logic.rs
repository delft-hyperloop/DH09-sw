//! The logic behind ethernet. Made using an FSM that checks the SOCKET status

use embassy_net::tcp::{TcpSocket, State};
use embassy_stm32::eth::{Ethernet, GenericPhy, PacketQueue};
use embassy_stm32::Peripherals;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use static_cell::StaticCell;

// static mut SOCKET: Mutex<NoopRawMutex, TcpSocket<'static>> = Mutex::new();

pub struct GsMaster {
    socket: TcpSocket<'static>
}

impl GsMaster {
    pub async fn init(p: Peripherals) -> Self {
        let mac_addr = lib::config::POD_MAC_ADDRESS;

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

        // Get an IPv4 address
        let config = embassy_net::Config::dhcpv4(Default::default());


    }
    
    pub async fn run(&self) -> ! {
        loop {
            let state: embassy_net::tcp::State = self.socket.state();

            match state {
                State::Closed | State::Closing | State::CloseWait => {
                    self.reconnect().await;
                },
                State::Established => {
                    self.transmit().await;
                    self.receive().await;
                }
                // and other states, I haven't looked into them
                _ => {}
            }
        }
    }

    pub async fn connect(&self) {
        todo!()
    }

    pub async fn reconnect(&self) {
        todo!()
    }

    pub async fn transmit(&self) {
        todo!()
    }

    pub async fn receive(&self) {
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