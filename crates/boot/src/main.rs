#![no_std]
#![no_main]

use core::borrow::BorrowMut;
use core::cell::RefCell;
use core::mem::MaybeUninit;
use core::num;
use core::assert;

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

#[embassy_executor::task(pool_size=1)]
async fn can_sender(pin: AnyPin, can_sender: BufferedFdCanSender) {
        let mut led = Output::new(pin, Level::Low, Speed::Low);

        loop {
            led.set_high();
            Timer::after_millis(100).await;
            led.set_low();
            Timer::after_millis(100).await;
        }
}

bind_interrupts!(struct CanOneInterrupts {
    FDCAN1_IT0 => can::IT0InterruptHandler<FDCAN1>;
    FDCAN1_IT1 => can::IT1InterruptHandler<FDCAN1>;
});

static mut read_buffer: RxFdBuf<{1<<3} > = RxFdBuf::new();
static mut write_buffer: TxFdBuf<{1<<3} > = TxFdBuf::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {   

    let mut config = Config::default();
    // config.rcc.hse = Some(rcc::Hse {
    //     freq: embassy_stm32::time::Hertz(25_000_000),
    //     mode: rcc::HseMode::Oscillator,
    // });
    // config.rcc.mux.fdcansel = rcc::mux::Fdcansel::HSE;

    let p = embassy_stm32::init(config);

    let mut can_config = CanConfigurator::new(p.FDCAN1, p.PD0, p.PD1, CanOneInterrupts);

    hprintln!("Ca;c: {}", 1_000_000);

    can_config.set_fd_data_bitrate(1u32 << 19, false);

    let can = can_config.into_normal_mode();

    let can = unsafe { can.buffered_fd(&mut write_buffer, &mut read_buffer) };

    spawner.must_spawn(can_sender(p.PB0.degrade(), can.writer()));
}
