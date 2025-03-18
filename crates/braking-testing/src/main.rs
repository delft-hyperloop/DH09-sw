
#![no_std]
#![no_main]
#![allow(warnings)]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_time::{Instant, Timer};
use {defmt_rtt as _, panic_probe as _};
use defmt::*;
use embassy_stm32::Peripherals;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // Init GPIO
    let button = Input::new(p.PC13, Pull::None);
    let mut ledBraking = Output::new(p.PB0, Level::Low, Speed::Low);
    let mut ledLatch = Output::new(p.PE1, Level::Low, Speed::Low);
    let mut braking = Output::new(p.PA3, Level::High, Speed::Low);
    let mut rearm = Output::new(p.PD7, Level::Low, Speed::Low);

    // Start with brakes and latch engaged
    braking.set_high();
    ledBraking.set_high();

    rearm.set_low();
    ledLatch.set_low();

    const LONG_PRESS_DURATION: u64 = 500; // 1 second
    loop {
        // // %%% This works %%% 
        // info!("Toggled braking (low)");
        // braking.toggle();
        // Timer::after_millis(1000).await;

        // info!("Toggled braking (high)");
        // braking.toggle();
        // Timer::after_millis(1000).await;

        let mut isLongPress = false;
        let mut buttonPressed = false;

        if button.is_high() {
            buttonPressed = true;
            let press_start = Instant::now();

            while button.is_high() {
                if press_start.elapsed().as_millis() > LONG_PRESS_DURATION {
                    isLongPress = true;
                    rearm.set_high();
                    ledLatch.set_high(); // Turn LED on
                }

                Timer::after_millis(10).await;
            }

            rearm.set_low();
            ledLatch.set_low(); // Turn LED off
        } 

        if buttonPressed & !isLongPress {
            info!("Short press!");
            ledBraking.toggle(); // Turn LED off
            braking.toggle(); // Disengage brakes
            if braking.is_set_high() {
                info!("Brakes disengaged");
            } else {
                info!("Brakes engaged");
            }
        }

        while button.is_high() {
            Timer::after_millis(10).await;
        }


        Timer::after_millis(20).await; // Debounce delay
    }
}
