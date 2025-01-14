//! A small executable to listen for CAN messages and dump them over RTT.
//! 
//! This is to check that the other PCBs on the CAN bus send the
//! correct messages (and in the correct order).

#![no_std]
#![no_main]

use core::num::NonZeroU8;
use core::num::NonZeroU16;

// use embassy_stm32::rng;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::Ipv4Address;
use embassy_net::StackResources;
use embassy_net::tcp::TcpSocket;
use embassy_stm32::bind_interrupts;
use embassy_stm32::can;
use embassy_stm32::can::RxFdBuf;
use embassy_stm32::can::TxFdBuf;
use embassy_stm32::can::config::DataBitTiming;
use embassy_stm32::can::config::NominalBitTiming;
use embassy_stm32::can::config::{self};
use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::PacketQueue;
use embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32::eth::{self};
use embassy_stm32::peripherals;
use embassy_stm32::peripherals::*;
use embassy_stm32::rcc;
use embassy_stm32::rng::Rng;
use embassy_stm32::rng::{self};
use embassy_sync::pubsub::PubSubBehavior;
use embassy_time::Timer;
use embedded_io_async::Write;
use panic_probe as _;
use rand_core::RngCore as _;
use static_cell::StaticCell;

bind_interrupts!(
    struct Irqs {
        // CAN
        FDCAN1_IT0 => can::IT0InterruptHandler<peripherals::FDCAN1>;
        FDCAN1_IT1 => can::IT1InterruptHandler<peripherals::FDCAN1>;

        FDCAN2_IT0 => can::IT0InterruptHandler<peripherals::FDCAN2>;
        FDCAN2_IT1 => can::IT1InterruptHandler<peripherals::FDCAN2>;
    }
);

fn hlt() -> ! {
    loop {
        cortex_m::asm::wfe();
    }
}

static mut read_buffer: RxFdBuf<5> = RxFdBuf::new();
static mut write_buffer: TxFdBuf<1> = TxFdBuf::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    defmt::println!("Hello, world!");

    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
        // config.rcc.hsi = Some(HSIPrescaler::DIV1);
        // config.rcc.csi = true;
        // config.rcc.hsi48 = Some(Default::default()); // needed for RNG
        // config.rcc.pll1 = Some(Pll {
        //     source: PllSource::HSI,
        //     prediv: PllPreDiv::DIV4,
        //     mul: PllMul::MUL50,
        //     divp: Some(PllDiv::DIV2),
        //     divq: None,
        //     divr: None,
        // });
        // config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        // config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        // config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        // config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        // config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        // config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        // config.rcc.voltage_scale = VoltageScale::Scale1;

        //// Config can

        config.rcc.hsi = Some(rcc::HSIPrescaler::DIV1);
        config.rcc.pll1 = Some(rcc::Pll {
            source: rcc::PllSource::HSI,
            prediv: rcc::PllPreDiv::DIV4,  // 64Mhz -> 16MHz
            mul: rcc::PllMul::MUL60,       // 16Mhz -> 960MHz,
            divp: Some(rcc::PllDiv::DIV2), // 960MHz -> 480MHz
            divq: Some(rcc::PllDiv::DIV8), // 960MHz -> 120MHz
            divr: None,
        });
        config.rcc.sys = rcc::Sysclk::PLL1_P; // 480MHz
        config.rcc.ahb_pre = rcc::AHBPrescaler::DIV2; // 240MHz to peripherals

        // Bump down peripheral clocks to 120MHz, which seems like the typical max
        // interface frequency and is mandated by Embassy
        config.rcc.apb1_pre = rcc::APBPrescaler::DIV2;
        config.rcc.apb2_pre = rcc::APBPrescaler::DIV2;
        config.rcc.apb3_pre = rcc::APBPrescaler::DIV2;
        config.rcc.apb4_pre = rcc::APBPrescaler::DIV2;

        // Voltage scaling 0 to support this
        config.rcc.voltage_scale = rcc::VoltageScale::Scale0;

        // 120MHz, must be equal to or less than APB1 bus
        config.rcc.mux.fdcansel = rcc::mux::Fdcansel::PLL1_Q;
        //
    }
    let p = embassy_stm32::init(config);

    info!("Embassy initialized!");

    let mut configurator = can::CanConfigurator::new(p.FDCAN1, p.PD0, p.PD1, Irqs);

    // hprintln!("{:?}", configurator.config().nbtr);
    // hprintln!("{:?}", configurator.config().dbtr);
    //NominalBitTiming { prescaler: 12, seg1: 8, seg2: 1, sync_jump_width: 1 }
    // DataBitTiming { transceiver_delay_compensation: true, prescaler: 2, seg1: 8,
    // seg2: 1, sync_jump_width: 1 }

    let config = configurator
        .config()
        // Configuration for 1Mb/s
        .set_nominal_bit_timing(NominalBitTiming {
            prescaler: NonZeroU16::new(10).unwrap(),
            seg1: NonZeroU8::new(8).unwrap(),
            seg2: NonZeroU8::new(3).unwrap(),
            sync_jump_width: NonZeroU8::new(3).unwrap(),
        })
        // Configuration for 2Mb/s
        .set_data_bit_timing(DataBitTiming {
            transceiver_delay_compensation: true,
            prescaler: NonZeroU16::new(5).unwrap(),
            seg1: NonZeroU8::new(7).unwrap(),
            seg2: NonZeroU8::new(4).unwrap(),
            sync_jump_width: NonZeroU8::new(4).unwrap(),
        })
        .set_tx_buffer_mode(config::TxBufferMode::Priority)
        .set_frame_transmit(config::FrameTransmissionConfig::AllowFdCanAndBRS);

    configurator.set_config(config);

    // hprintln!("Generated config: {:?}", configurator.config());

    let mut can = configurator.into_normal_mode();

    // TODO: figure out if we want buffered can
    // let mut can = can.buffered_fd(unsafe{&mut write_buffer}, unsafe{&mut
    // read_buffer});

    // let mut can = can.into_internal_loopback_mode();
    // let mut can = can.into_normal_mode();

    info!("CAN Configured");

    loop {
        let response = can.read_fd().await;
        // hprintln!("Yoo");

        match response {
            Ok(ref envelope) => {
                let header = envelope.frame.header();
                info!("Received: {:?}", header);
            }
            Err(ref bus_error) => {
                info!("Bus error: {:?}", bus_error);
            }
        }
    }

    hlt()
}
