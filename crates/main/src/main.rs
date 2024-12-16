#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_net::{tcp::TcpSocket, Ipv4Address, StackResources};
use embassy_stm32::{
    eth::{self, generic_smi::GenericSMI, Ethernet, PacketQueue},
    peripherals,
    rng::{self, Rng},
};
// use embassy_stm32::rng;
use defmt::*;
use defmt_rtt as _;
use embassy_stm32::peripherals::*;
use embedded_io_async::Write;
use main::can::CanInterface;
use panic_probe as _;

use embassy_stm32::bind_interrupts;
use embassy_time::Timer;

use embassy_stm32::can;
use rand_core::RngCore as _;
use static_cell::StaticCell;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

bind_interrupts!(
    struct Irqs {
        ETH => eth::InterruptHandler;
        HASH_RNG => rng::InterruptHandler<peripherals::RNG>;

        // CAN
        FDCAN1_IT0 => can::IT0InterruptHandler<peripherals::FDCAN1>;
        FDCAN1_IT1 => can::IT1InterruptHandler<peripherals::FDCAN1>;
    }
);

fn hlt() -> ! {
    loop {
        cortex_m::asm::wfe();
    }
}

type Device = Ethernet<'static, ETH, GenericSMI>;

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    defmt::println!("Hello, world!");

    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = Some(HSIPrescaler::DIV1);
        config.rcc.csi = true;
        config.rcc.hsi48 = Some(Default::default()); // needed for RNG
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV2),
            divq: None,
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.voltage_scale = VoltageScale::Scale1;
    }
    let p = embassy_stm32::init(config);

    info!("Embassy initialized!");

    // let mut can = can::CanConfigurator::new(p.FDCAN1, p.PA11, p.PA12, Irqs);

    // // 250k bps
    // can.set_bitrate(250_000);

    // // let mut can = can.into_internal_loopback_mode();
    // let mut can = can.into_normal_mode();

    // let can = CanInterface::new(can, spawner);

    info!("CAN Configured");

    // Generate random seed.
    let mut rng = Rng::new(p.RNG, Irqs);
    let mut seed = [0; 8];
    rng.fill_bytes(&mut seed);
    let seed = u64::from_le_bytes(seed);

    let mac_addr = [0x00, 0x07, 0xE9, 0x42, 0xAC, 0x28];

    static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
    // warning: Not all STM32H7 devices have the exact same pins here
    // for STM32H747XIH, replace p.PB13 for PG12
    let device = Ethernet::new(
        PACKETS.init(PacketQueue::<4, 4>::new()),
        p.ETH,
        Irqs,
        p.PA1,  // ref_clk
        p.PA2,  // mdio
        p.PC1,  // eth_mdc
        p.PA7,  // CRS_DV: Carrier Sense
        p.PC4,  // RX_D0: Received Bit 0
        p.PC5,  // RX_D1: Received Bit 1
        p.PG13, // TX_D0: Transmit Bit 0
        p.PB13, // TX_D1: Transmit Bit 1
        p.PG11, // TX_EN: Transmit Enable
        GenericSMI::new(0),
        mac_addr,
    );

    let config = embassy_net::Config::dhcpv4(Default::default());
    // let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
    //    address: Ipv4Cidr::new(Ipv4Address::new(10, 42, 0, 61), 24),
    //    dns_servers: Vec::new(),
    //    gateway: Some(Ipv4Address::new(10, 42, 0, 1)),
    // });

    // Init network stack
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) =
        embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

    // Launch network task
    spawner.spawn(net_task(runner)).unwrap();

    // Ensure DHCP configuration is up before trying connect
    stack.wait_config_up().await;

    info!("Hello!");

    // Then we can use it!
    let mut rx_buffer = [0; 8192];
    let mut tx_buffer = [0; 8192];

    let mut to_write = [b'H'; 8192];
    *to_write.last_mut().unwrap() = b'\n';

    loop {
        info!("Trying!");

        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

        // You need to start a server on the host machine, for example: `nc -l 8000`
        let remote_endpoint = (Ipv4Address::new(192, 168, 1, 17), 8000);
        let r = socket.connect(remote_endpoint).await;
        if let Err(e) = r {
            error!("{}", e);
            // hprintln!("connect error: {:?}", e);
            Timer::after_secs(1).await;
            continue;
        }
        // hprintln!("connected!");

        let start_instant = embassy_time::Instant::now();

        unwrap!(socket.write_all(&to_write).await);
        unwrap!(socket.flush().await);

        let end_instant = embassy_time::Instant::now();

        let diff = end_instant - start_instant;

        info!("Wrote {} bytes in {}us", to_write.len(), diff.as_micros());

        socket.close();
    }

    hlt()
}
