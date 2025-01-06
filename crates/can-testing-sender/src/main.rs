#![no_std]
#![no_main]

use core::arch::asm;
use core::assert;
use core::borrow::BorrowMut;
use core::cell::RefCell;
use core::future::Future;
use core::mem::MaybeUninit;
use core::num;
use core::num::NonZero;
use core::num::NonZeroU16;
use core::num::NonZeroU8;
use core::time;

use cortex_m::itm;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayUs;
use cortex_m::Peripherals;
use defmt::*;
use defmt_rtt as _;
use defmt_rtt;
// use cortex_m_semihosting::hprint;
use embassy_boot::BootLoaderConfig;
use embassy_executor::Spawner;
use embassy_futures::yield_now;
use embassy_stm32::bind_interrupts;
use embassy_stm32::can;
use embassy_stm32::can::config;
use embassy_stm32::can::config::DataBitTiming;
use embassy_stm32::can::config::NominalBitTiming;
use embassy_stm32::can::config::TimestampPrescaler;
use embassy_stm32::can::frame::FdEnvelope;
use embassy_stm32::can::frame::FdFrame;
use embassy_stm32::can::frame::Header;
use embassy_stm32::can::frame::Id;
use embassy_stm32::can::BufferedFdCanSender;
use embassy_stm32::can::CanConfigurator;
use embassy_stm32::can::RxFdBuf;
use embassy_stm32::can::TxFdBuf;
use embassy_stm32::gpio::AnyPin;
use embassy_stm32::gpio::Input;
use embassy_stm32::gpio::Level;
use embassy_stm32::gpio::Output;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::gpio::Pin;
use embassy_stm32::gpio::Pull;
use embassy_stm32::gpio::Speed;
use embassy_stm32::lptim::pwm::Ch1;
use embassy_stm32::peripherals::FDCAN1;
use embassy_stm32::peripherals::TIM1;
use embassy_stm32::peripherals::TIM3;
use embassy_stm32::peripherals::{self};
use embassy_stm32::rcc;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::simple_pwm::PwmPin;
use embassy_stm32::timer::simple_pwm::SimplePwm;
use embassy_stm32::timer::Channel;
use embassy_stm32::Config;
use embassy_time::Delay;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::Ticker;
use embassy_time::Timer;
use panic_probe as _;
// pick a panicking behavior

fn cpu_freq() -> f32 {
    let mut dwt = cortex_m::Peripherals::take().unwrap().DWT;

    dwt.enable_cycle_counter();

    let start_count = dwt.cyccnt.read();
    let start_time = Instant::now();

    let mut count: u32 = 100000000;
    unsafe {
        asm!(
            "0:
            subs {0}, {0}, #1
            bne 0b
            ",
            inout(reg) count
        );
    }

    let end_time = Instant::now();
    let end_count = dwt.cyccnt.read();
    let time_passed = end_time - start_time;

    dwt.disable_cycle_counter();

    ((end_count - start_count) as f32) / (time_passed.as_micros() as f32)
}

fn generate_config() -> Config {
    let mut config = Config::default();

    // Config
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

    config
}

bind_interrupts!(struct CanOneInterrupts {
    FDCAN1_IT0 => can::IT0InterruptHandler<FDCAN1>;
    FDCAN1_IT1 => can::IT1InterruptHandler<FDCAN1>;
});

static mut read_buffer: RxFdBuf<5> = RxFdBuf::new();
static mut write_buffer: TxFdBuf<1> = TxFdBuf::new();

const DELAY: u32 = 300_000;

#[embassy_executor::task]
async fn blocking_blink(led: AnyPin) {
    let mut led = Output::new(led, Level::Low, Speed::Medium);
    let mut delay: Delay = Delay;
    let mut ticker = Ticker::every(Duration::from_secs(1));
    loop {
        led.set_high();
        delay.delay_us(DELAY as u32);
        led.set_low();

        ticker.next().await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(generate_config());

    // #[cfg(feature = "read")]
    // spawner.must_spawn(blocking_blink(p.PB0.degrade()));

    let mut configurator = CanConfigurator::new(p.FDCAN1, p.PD0, p.PD1, CanOneInterrupts);

    // hprintln!("{:?}", configurator.config().nbtr);
    // hprintln!("{:?}", configurator.config().dbtr);
    //NominalBitTiming { prescaler: 12, seg1: 8, seg2: 1, sync_jump_width: 1 }
    // DataBitTiming { transceiver_delay_compensation: true, prescaler: 2, seg1: 8,
    // seg2: 1, sync_jump_width: 1 }

    configurator.set_bitrate(250_000);

    // let config = configurator
    //     .config()
    //     // Configuration for 1Mb/s
    //     .set_nominal_bit_timing(NominalBitTiming {
    //         prescaler: NonZeroU16::new(10).unwrap(),
    //         seg1: NonZeroU8::new(8).unwrap(),
    //         seg2: NonZeroU8::new(3).unwrap(),
    //         sync_jump_width: NonZeroU8::new(3).unwrap(),
    //     })
    //     // Configuration for 2Mb/s
    //     .set_data_bit_timing(DataBitTiming {
    //         transceiver_delay_compensation: true,
    //         prescaler: NonZeroU16::new(5).unwrap(),
    //         seg1: NonZeroU8::new(7).unwrap(),
    //         seg2: NonZeroU8::new(4).unwrap(),
    //         sync_jump_width: NonZeroU8::new(4).unwrap(),
    //     })
    //     .set_tx_buffer_mode(config::TxBufferMode::Priority)
    //     .set_frame_transmit(config::FrameTransmissionConfig::AllowFdCanAndBRS);

    // configurator.set_config(config);

    // hprintln!("Generated config: {:?}", configurator.config());

    let mut can = configurator.into_normal_mode();

    // #[cfg(feature = "read")]
    // let mut can = can.buffered_fd(unsafe { &mut write_buffer }, unsafe { &mut read_buffer });

    // let frame = FdFrame::new_extended(0x0001,
    //     &[0xFF; 64]).expect("Frame build error");
    let header = Header::new_fd(
        Id::try_from(0x00000001 as u32).expect("Invalid ID"),
        64,
        false,
        true,
    );

    let frame = FdFrame::new(header, &[0; 64]).expect("Invalid frame");

    loop {
        defmt::info!("Wrote frame");
        can.write_fd(&frame).await;
    }

    // hprintln!("CPU Freq: {:.0}MHz", cpu_freq());
}
