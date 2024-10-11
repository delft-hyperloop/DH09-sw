#![no_std]
#![no_main]

#[cfg(all(feature = "rtt", feature = "qemu"))]
compile_error!("The `rtt` and `qemu` features are mutually exclusive");

use embassy_executor::Spawner;
use embassy_stm32::peripherals;
// use embassy_stm32::rng;
// pick a panicking behavior
// use panic_halt as _;
use defmt::*;
#[cfg(feature = "rtt")]
use defmt_rtt as _;
#[cfg(feature = "qemu")]
use defmt_semihosting as _;
use main::can::CanInterface;
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use panic_probe as _;
use embassy_stm32::peripherals::*;

use embassy_stm32::bind_interrupts;
use embassy_time::Timer;

use embassy_stm32::can;

bind_interrupts!(
    struct Irqs {
        // RNG => rng::InterruptHandler<peripherals::RNG>;

        // CAN
        FDCAN1_IT0 => can::IT0InterruptHandler<peripherals::FDCAN1>;
        FDCAN1_IT1 => can::IT1InterruptHandler<peripherals::FDCAN1>;
    }
);

fn hlt() -> ! {
    #[cfg(feature = "qemu")]
    {
        use core::arch::asm;

        #[allow(non_upper_case_globals)]
        const ADP_Stopped_ApplicationExit: u32 = 0x20026;

        #[repr(C)]
        struct QEMUParameterBlock {
            arg0: u32,
            arg1: u32,
        }

        let block = QEMUParameterBlock {
            arg0: ADP_Stopped_ApplicationExit,
            arg1: 0,
        };

        unsafe {
            asm!(
                "bkpt #0xab",
                in("r0") 0x20,
                in("r1") &block as *const _ as u32,
                options(nostack)
            );

            loop {
                cortex_m::asm::wfe();
            }
        }
    }

    #[cfg(feature = "rtt")]
    {
        loop {
            cortex_m::asm::wfe();
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    trace!("Hello, world!");

    let config = embassy_stm32::Config::default();
    let p = embassy_stm32::init(config);

    info!("Embassy initialized!");

    // let mut can = can::CanConfigurator::new(p.FDCAN1, p.PA11, p.PA12, Irqs);

    // // 250k bps
    // can.set_bitrate(250_000);

    // // let mut can = can.into_internal_loopback_mode();
    // let mut can = can.into_normal_mode();

    // let can = CanInterface::new(can, spawner);

    info!("CAN Configured");

    hlt()
}
