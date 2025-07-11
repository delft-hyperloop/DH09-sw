//! a panic handler for release builds of the main pcb code.
//!
//! on panic, it first triggers emergency brakes,
//! and then after waiting a bit,
//! reboots the device.
use core::panic::PanicInfo;

use cortex_m::peripheral::SCB;
use embassy_stm32::gpio::Level;
use embassy_stm32::gpio::Output;
use embassy_stm32::gpio::Speed;

/// a panic handler for release builds of the main pcb.
/// on panic, it first triggers emergency brakes,
/// and then after waiting a bit,
/// reboots the device.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 1. trigger emergency brakes
    // SAFETY: we are in a panic state, so this function is triggered as an
    // interrupt, and control will never be given back to anything else, so
    // using peripherals is always safe.
    unsafe {
        let peripherals = embassy_stm32::Peripherals::steal();
        let mut sdc = Output::new(peripherals.PB0, Level::Low, Speed::Medium);
        sdc.set_low();
    }
    // 2. print info
    defmt::error!("[[PANIC!]]");
    defmt::error!("caused by: {:?}", info);

    // 3. wait.
    // * stm32h743zit6u runs at ~400MHz, with a max of 480MHz,
    // * we want to wait for ~30 seconds before rebooting,
    //
    // + subs r0, r0, #1   ; 1 cycle
    // + bne 1b            ; ~2 cycles
    cortex_m::asm::delay(4_000_000_000);

    // 4. reset pcb
    SCB::sys_reset()
}
