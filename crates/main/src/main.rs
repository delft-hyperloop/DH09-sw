#![cfg_attr(not(test), no_std)]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{
    eth::{self, generic_smi::GenericSMI, Ethernet},
    peripherals,
    rng::{self},
};
// use embassy_stm32::rng;
use defmt::*;
use defmt_rtt as _;
use embassy_stm32::peripherals::*;
use panic_probe as _;

use embassy_stm32::bind_interrupts;
use embassy_stm32::gpio::{Level, Output, Speed, Input, Pull};
use embassy_time::{Duration, Ticker};

use embassy_stm32::can;

use fsm::commons::traits::Runner;
use fsm::commons::EmergencyChannel;
use fsm::commons::EventChannel;
use fsm::MainFSM;

// For making polling into an async
use core::future::poll_fn;
use core::task::Poll;

// use main::can::CanInterface;
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use panic_probe as _;


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

// Initialize the channel for publishing events to the FSMs.
static EVENT_CHANNEL: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
static EMERGENCY_CHANNEL: static_cell::StaticCell<EmergencyChannel> =
    static_cell::StaticCell::new();

#[embassy_executor::task]
async fn run_fsm(
    spawner: Spawner,
    event_channel: &'static EventChannel,
    emergency_channel: &'static EmergencyChannel,
) {
    let mut main_fsm = MainFSM::new(spawner, event_channel, emergency_channel);
    main_fsm.run().await;
}

/// Number of bits in SSI data package.
const DATA_LENGTH: u32 = 41; // 24 (position) + 16 (speed) + 1 (valid)

#[embassy_executor::task]
async fn ssi_ticker(mut clock: Output<'static>, mut clock_differential: Output<'static>, data: Input<'static>, data_differential: Input<'static>) {
    // Half of the period, value should be changed
    let mut ticker = Ticker::every(Duration::from_millis(1));
    
    loop {
        // IDLE clock state
        clock.set_high();
        clock_differential.set_high(); 

        // Wait for data to be ready (both data high), this allows other processes to run as well
        poll_fn(|ctx| {
            if data.is_low() && data_differential.is_low() {
                return Poll::Ready(());
            }
            // We want to be polled ASAP, later this can be based on an interrupt of the GPIO pins
            ctx.waker().clone().wake(); 
            return Poll::Pending;
        }).await;

        // Holds enough bits for our usecase (=41bits)
        let mut received = 0u64;
        let mut error = false;

        // Transmission hasn't started yet, that happens at first rising edge
        clock.set_low();
        clock_differential.set_high(); 

        // Start receiving each packet?
        for i in 0..DATA_LENGTH {
            clock.set_high();
            clock_differential.set_low();

            ticker.next().await;

            if data.is_high() && data_differential.is_high() || data.is_low() && data_differential.is_low() {
                // Bit flipped, skip message?
                error = true;
                break;
            } else {
                received |= (if data.is_high() {1} else {0}) << i;
            }

            clock.set_low();
            clock_differential.set_high();
            ticker.next().await;
        }

        // Send another clock to check that we did reach the end of transmission
        clock.set_high();
        clock_differential.set_low();

        if !(data.is_low() && data_differential.is_low()) {
            error = true;
        }

        if error {
            log!("No bueno");
        } else {
            // We also need to split the result and decode:
            // Decoding involves extending the 24bits of the position into a signed 32bit
            log!("Received: {}", received);
        }

        // What to store value in? Embassy pipe may not be good since SSI is supposed to be real fast and the pipe will wait for its buffer to be freed.
    }
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

    // I picked random pins for this, feel free to change them
    let clock1 = Output::new(p.PA0, Level::High, Speed::Low);
    let clock2 = Output::new(p.PA1, Level::Low, Speed::Low);
    let data1 = Input::new(p.PA2, Pull::Down);
    let data2 = Input::new(p.PA3, Pull::Down);

    spawner.spawn(ssi_ticker(clock1, clock2, data1, data2)).unwrap();

    // let event_channel = EVENT_CHANNEL.init(EventChannel::new());
    // let emergency_channel = EMERGENCY_CHANNEL.init(EmergencyChannel::new());

    info!("Embassy initialized!");

    // let mut can = can::CanConfigurator::new(p.FDCAN1, p.PA11, p.PA12, Irqs);

    // // 250k bps
    // can.set_bitrate(250_000);

    // // let mut can = can.into_internal_loopback_mode();
    // let mut can = can.into_normal_mode();

    // let can = CanInterface::new(can, spawner);

    info!("CAN Configured");

    loop {}

    // spawner
    //     .spawn(run_fsm(spawner, event_channel, emergency_channel))
    //     .unwrap();

    // info!("FSM started!");

    // // Generate random seed.
    // let mut rng = Rng::new(p.RNG, Irqs);
    // let mut seed = [0; 8];
    // rng.fill_bytes(&mut seed);
    // let seed = u64::from_le_bytes(seed);

    // let mac_addr = [0x00, 0x07, 0xE9, 0x42, 0xAC, 0x28];

    // static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
    // // warning: Not all STM32H7 devices have the exact same pins here
    // // for STM32H747XIH, replace p.PB13 for PG12
    // let device = Ethernet::new(
    //     PACKETS.init(PacketQueue::<4, 4>::new()),
    //     p.ETH,
    //     Irqs,
    //     p.PA1,  // ref_clk
    //     p.PA2,  // mdio
    //     p.PC1,  // eth_mdc
    //     p.PA7,  // CRS_DV: Carrier Sense
    //     p.PC4,  // RX_D0: Received Bit 0
    //     p.PC5,  // RX_D1: Received Bit 1
    //     p.PG13, // TX_D0: Transmit Bit 0
    //     p.PB13, // TX_D1: Transmit Bit 1
    //     p.PG11, // TX_EN: Transmit Enable
    //     GenericSMI::new(0),
    //     mac_addr,
    // );

    // let config = embassy_net::Config::dhcpv4(Default::default());
    // // let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
    // //    address: Ipv4Cidr::new(Ipv4Address::new(10, 42, 0, 61), 24),
    // //    dns_servers: Vec::new(),
    // //    gateway: Some(Ipv4Address::new(10, 42, 0, 1)),
    // // });

    // // Init network stack
    // static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    // let (stack, runner) =
    //     embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

    // // Launch network task
    // spawner.spawn(net_task(runner)).unwrap();

    // // Ensure DHCP configuration is up before trying connect
    // stack.wait_config_up().await;

    // info!("Hello!");

    // // Then we can use it!
    // let mut rx_buffer = [0; 8192];
    // let mut tx_buffer = [0; 8192];

    // let mut to_write = [b'H'; 8192];
    // *to_write.last_mut().unwrap() = b'\n';

    // loop {
    //     info!("Trying!");

    //     let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
    //     socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

    //     // You need to start a server on the host machine, for example: `nc -l 8000`
    //     let remote_endpoint = (Ipv4Address::new(192, 168, 1, 17), 8000);
    //     let r = socket.connect(remote_endpoint).await;
    //     if let Err(e) = r {
    //         error!("{}", e);
    //         // hprintln!("connect error: {:?}", e);
    //         Timer::after_secs(1).await;
    //         continue;
    //     }
    //     // hprintln!("connected!");

    //     let start_instant = embassy_time::Instant::now();

    //     unwrap!(socket.write_all(&to_write).await);
    //     unwrap!(socket.flush().await);

    //     let end_instant = embassy_time::Instant::now();

    //     let diff = end_instant - start_instant;

    //     info!("Wrote {} bytes in {}us", to_write.len(), diff.as_micros());

    //     socket.close();
    // }

    // hlt()
}
