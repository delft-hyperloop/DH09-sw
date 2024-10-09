#![no_std]
#![no_main]

#[cfg(all(feature = "rtt", feature = "qemu"))]
compile_error!("The `rtt` and `qemu` features are mutually exclusive");

use embassy_executor::Spawner;
use embassy_stm32::peripherals;
use embassy_stm32::rng;
// pick a panicking behavior
// use panic_halt as _;
use defmt::*;
#[cfg(feature = "rtt")]
use defmt_rtt as _;
#[cfg(feature = "qemu")]
use defmt_semihosting as _;
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use panic_probe as _;

use cortex_m_semihosting::hprintln;
use embassy_stm32::bind_interrupts;

use embassy_stm32::can;

bind_interrupts!(
    struct Irqs {
        RNG => rng::InterruptHandler<peripherals::RNG>;

        // CAN
        FDCAN1_IT0 => can::IT0InterruptHandler<peripherals::FDCAN1>;
        FDCAN1_IT1 => can::IT1InterruptHandler<peripherals::FDCAN1>;
    }
);

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    trace!("Hello, world!");

    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
    }
    let p = embassy_stm32::init(config);

    loop {}
}
