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

    let mut configurator = can::CanConfigurator::new(p.FDCAN2, p.PB5, p.PB6, Irqs);

    configurator.set_bitrate(500_000);

    // configurator.set_config(config);

    // hprintln!("Generated config: {:?}", configurator.config());

    let mut can = configurator.into_normal_mode();

    // TODO: figure out if we want buffered can
    // let mut can = can.buffered_fd(unsafe{&mut write_buffer}, unsafe{&mut
    // read_buffer});

    // let mut can = can.into_internal_loopback_mode();
    // let mut can = can.into_normal_mode();

    info!("CAN Configured");

    fn decode_temperature(encoded: u8) -> f32 {
        let precision_range_start: f32 = 20.0;

        if encoded & 0x80 != 0 {
            precision_range_start + ((encoded & 0x7F) as f32 / 10.0) // High precision mode
        } else {
            encoded as f32 // Integer mode
        }
    }

    loop {
        info!("Waiting for CAN Frame...");
        let response = can.read().await;

        match response {
            Ok(ref envelope) => {
                let header = envelope.frame.header();
                let data = envelope.frame.data(); // Extract received data

                info!("Received CAN Frame: {:?} ({:?})", header, data);

                if data.len() >= 1 {
                    // Ensure at least 1 byte received
                    let received_temp = decode_temperature(data[0]);
                    info!("Decoded Temperature: {} °C", received_temp);
                } else {
                    info!("Received frame but no valid temperature data!");
                }
            }
            Err(ref bus_error) => {
                error!("Bus error: {:?}", bus_error);
            }
        }
    }

    hlt()
}