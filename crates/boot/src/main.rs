#![no_std]
#![no_main]

use core::borrow::BorrowMut;
use core::cell::RefCell;
use core::mem::MaybeUninit;
use core::num;
use core::assert;
use core::time;

use embassy_boot::BootLoaderConfig;
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
use cortex_m::Peripherals;
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

#[embassy_executor::main]
async fn main(spawner: Spawner) {   

    let mut config = Config::default();
    
    // Config
    config.rcc.hsi = Some(rcc::HSIPrescaler::DIV2);
    //


    let p = embassy_stm32::init(config);

    hprintln!("CPU Freq: {}", cpu_freq());

}   
