#![no_std]
#![no_main]

use core::cell::RefCell;

use embassy_boot::BootLoaderConfig;
use embassy_executor::Spawner;
use embassy_stm32::flash::{Bank1Region, Blocking, BANK1_REGION};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
// pick a panicking behavior
use panic_halt as _;
// use defmt::*;
// use defmt_rtt as _;
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_semihosting::hprintln;
use embassy_boot_stm32::BootLoader;
use embassy_boot_stm32::{BlockingFirmwareUpdater, FirmwareUpdaterConfig};
use embassy_stm32::{bind_interrupts, flash::Flash};

bind_interrupts!(
    struct Irqs {}
);

unsafe fn get_bootloader_state_slice() -> &'static mut [u8] {
    extern "C" {
        static __bootloader_state_start: u8;
        static __bootloader_state_end: u8;
    }
    let len = &__bootloader_state_end as *const u8 as usize
        - &__bootloader_state_start as *const u8 as usize;
    unsafe { core::slice::from_raw_parts_mut(__bootloader_state_end as *mut u8, len) }
}

// unsafe fn get_bootloader_flash_slice() -> &'static mut [u8] {
//     extern "C" {
//         static __bootloader_active_start: u8;
//         static __bootloader_active_end: u8;
//     }
//     let len = &__bootloader_active_end as *const u8 as usize
//         - &__bootloader_active_start as *const u8 as usize;
//     unsafe { core::slice::from_raw_parts_mut(__bootloader_active_start as *mut u8, len) }
// }

type FlashRef<'a> = Mutex<NoopRawMutex, RefCell<Bank1Region<'a, Blocking>>>;

unsafe fn boot_firmware(flash: &FlashRef) -> ! {
    let bootloader_config = BootLoaderConfig::from_linkerfile_blocking(&flash, &flash, &flash);
    let offset = bootloader_config.active.offset();

    let bl = BootLoader::prepare::<_, _, _, 8192>(bootloader_config);
    let start = BANK1_REGION.base + offset;

    unsafe { bl.load(start) }
}

unsafe fn update_firmware(flash: &FlashRef) {
    let config = FirmwareUpdaterConfig::from_linkerfile_blocking(&flash, &flash);
    let bootloader_state = unsafe { get_bootloader_state_slice() };
    let fwu = BlockingFirmwareUpdater::new(config, bootloader_state);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    hprintln!("Hello, world!");

    let p = embassy_stm32::init(Default::default());
    let layout = Flash::new_blocking(p.FLASH).into_blocking_regions();
    let flash = Mutex::new(RefCell::new(layout.bank1_region));

    unsafe { boot_firmware(&flash) }
}
