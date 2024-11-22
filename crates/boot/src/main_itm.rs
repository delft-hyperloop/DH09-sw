#![no_std]
#![no_main]

use core::borrow::BorrowMut;
use core::cell::RefCell;
use core::mem::MaybeUninit;
use core::num;
use core::assert;
use core::num::NonZero;
use core::num::NonZeroU16;
use core::num::NonZeroU8;
use core::time;

use cortex_m_semihosting::hprint;
use embassy_boot::BootLoaderConfig;
use embassy_stm32::can::config;
use embassy_stm32::can::config::DataBitTiming;
use embassy_stm32::can::config::NominalBitTiming;
use embassy_stm32::can::config::TimestampPrescaler;
use embassy_stm32::can::frame::FdFrame;
use embassy_stm32::{bind_interrupts, can, rcc, Config};
use embassy_executor::Spawner;
use embassy_stm32::can::{TxFdBuf, RxFdBuf};
use embassy_stm32::can::{BufferedFdCanSender, CanConfigurator};
use embassy_stm32::lptim::pwm::Ch1;
use embassy_stm32::peripherals::{self, TIM1, TIM3, FDCAN1};
use embassy_stm32::time::{Hertz};
use embassy_stm32::timer::{Channel};
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed, OutputType};
use embassy_time::{Duration, Timer, Instant};
use core::arch::asm;
use cortex_m;
// pick a panicking behavior

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use cortex_m_semihosting::{hprintln};

macro_rules! debug_hprintln {
    ($($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                hprintln!($($e),+)
            }
        }
    };
}

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

    ((end_count - start_count) as f32)/(time_passed.as_micros() as f32)
}

fn generate_config() -> Config {
    let mut config = Config::default();
    
    // Config
    config.rcc.hsi = Some(rcc::HSIPrescaler::DIV1);
    config.rcc.pll1 = Some(
        rcc::Pll {
            source: rcc::PllSource::HSI,
            prediv: rcc::PllPreDiv::DIV4, // 64Mhz -> 16MHz
            mul: rcc::PllMul::MUL60, // 16Mhz -> 960MHz,
            divp: Some(rcc::PllDiv::DIV2), // 960MHz -> 480MHz
            divq: Some(rcc::PllDiv::DIV8), // 960MHz -> 120MHz
            divr: None
        }
    );
    config.rcc.sys = rcc::Sysclk::PLL1_P; // 480MHz
    config.rcc.ahb_pre = rcc::AHBPrescaler::DIV2; // 240MHz to peripherals

    // Bump down peripheral clocks to 120MHz, which seems like the typical max interface frequency and is mandated by Embassy
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

static mut read_buffer: RxFdBuf<{1<<3} > = RxFdBuf::new();
static mut write_buffer: TxFdBuf<{1<<3} > = TxFdBuf::new();


#[embassy_executor::main]
async fn main(spawner: Spawner) {   
    let p = embassy_stm32::init(generate_config());
    
    let mut itm = cortex_m::Peripherals::take().unwrap().ITM;
    let stim = &mut itm.stim[0];

    cortex_m::itm::write_str(stim, "Testing, testing");
    
    // hprintln!("CPU Freq: {:.0}MHz", cpu_freq());
}   

