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

use embassy_time::Timer;
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
        use embassy_stm32::rcc;
        config.rcc.hse = Some(rcc::Hse {
            freq: embassy_stm32::time::Hertz(25_000_000),
            mode: rcc::HseMode::Oscillator,
        });
        config.rcc.mux.fdcansel = rcc::mux::Fdcansel::HSE;
    }
    let p = embassy_stm32::init(config);

    info!("Embassy initialized!");

    let mut can = can::CanConfigurator::new(p.FDCAN1, p.PA11, p.PA12, Irqs);

    // 250k bps
    can.set_bitrate(250_000);

    let mut can = can.into_internal_loopback_mode();
    // let mut can = can.into_normal_mode();

    info!("CAN Configured");

    let mut i = 0;
    let mut last_read_ts = embassy_time::Instant::now();

    loop {
        let frame = can::frame::Frame::new_extended(0x123456F, &[i; 8]).unwrap();
        info!("Writing frame");
        _ = can.write(&frame).await;

        match can.read().await {
            Ok(envelope) => {
                let (rx_frame, ts) = envelope.parts();
                let delta = (ts - last_read_ts).as_millis();
                last_read_ts = ts;
                info!(
                    "Rx: {:x} {:x} {:x} {:x} --- NEW {}",
                    rx_frame.data()[0],
                    rx_frame.data()[1],
                    rx_frame.data()[2],
                    rx_frame.data()[3],
                    delta,
                )
            }
            Err(_err) => error!("Error in frame"),
        }

        Timer::after_millis(250).await;

        i += 1;
        if i > 3 {
            break;
        }
    }

    let (mut tx, mut rx, _props) = can.split();
    // With split
    loop {
        let frame = can::frame::Frame::new_extended(0x123456F, &[i; 8]).unwrap();
        info!("Writing frame");
        _ = tx.write(&frame).await;

        match rx.read().await {
            Ok(envelope) => {
                let (rx_frame, ts) = envelope.parts();
                let delta = (ts - last_read_ts).as_millis();
                last_read_ts = ts;
                info!(
                    "Rx: {:x} {:x} {:x} {:x} --- NEW {}",
                    rx_frame.data()[0],
                    rx_frame.data()[1],
                    rx_frame.data()[2],
                    rx_frame.data()[3],
                    delta,
                )
            }
            Err(_err) => error!("Error in frame"),
        }

        Timer::after_millis(250).await;

        i = i.wrapping_add(1);
    }
}
