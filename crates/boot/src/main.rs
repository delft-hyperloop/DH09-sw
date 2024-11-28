#![no_std]
#![no_main]

use core::cell::RefCell;

// use defmt::*;
// use defmt_rtt as _;
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m_semihosting::hprintln;
use embassy_boot::BootLoaderConfig;
use embassy_boot_stm32::BlockingFirmwareUpdater;
use embassy_boot_stm32::BootLoader;
use embassy_boot_stm32::FirmwareUpdaterConfig;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::Ipv4Address;
use embassy_net::Ipv4Cidr;
use embassy_net::StackResources;
use embassy_stm32::bind_interrupts;
use embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::PacketQueue;
use embassy_stm32::eth::{self};
use embassy_stm32::flash::Bank1Region;
use embassy_stm32::flash::Blocking;
use embassy_stm32::flash::Flash;
use embassy_stm32::flash::BANK1_REGION;
use embassy_stm32::peripherals;
use embassy_stm32::peripherals::ETH;
use embassy_stm32::rng::Rng;
use embassy_stm32::rng::{self};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::Timer;
use embedded_io_async::Write;
use heapless::Vec;
// pick a panicking behavior
use panic_halt as _;
use rand_core::RngCore;
use static_cell::StaticCell;

bind_interrupts!(
    struct Irqs {
        ETH => eth::InterruptHandler;
        RNG => rng::InterruptHandler<peripherals::RNG>;
    }
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
//     unsafe { core::slice::from_raw_parts_mut(__bootloader_active_start as
// *mut u8, len) } }

type FlashRef<'a> = Mutex<NoopRawMutex, RefCell<Bank1Region<'a, Blocking>>>;

fn boot_firmware(flash: &FlashRef) -> ! {
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

type Device = Ethernet<'static, ETH, GenericSMI>;

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    hprintln!("Hello, world!");

    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = Some(HSIPrescaler::DIV1);
        config.rcc.csi = true;
        config.rcc.hsi48 = Some(Default::default()); // needed for RNG
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV2),
            divq: None,
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.voltage_scale = VoltageScale::Scale1;
    }
    let p = embassy_stm32::init(config);
    let layout = Flash::new_blocking(p.FLASH).into_blocking_regions();
    let flash = Mutex::new(RefCell::new(layout.bank1_region));

    // Generate random seed.
    let mut rng = Rng::new(p.RNG, Irqs);
    let mut seed = [0; 8];
    rng.fill_bytes(&mut seed);
    let seed = u64::from_le_bytes(seed);

    let mac_addr = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];
    hprintln!("Connecting to ETH");

    static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
    // warning: Not all STM32H7 devices have the exact same pins here
    // for STM32H747XIH, replace p.PB13 for PG12
    let device = Ethernet::new(
        PACKETS.init(PacketQueue::<4, 4>::new()),
        p.ETH,
        Irqs,
        p.PA1,  // ref_clk
        p.PA2,  // mdio
        p.PC1,  // eth_mdc
        p.PA7,  // CRS_DV: Carrier Sense
        p.PC4,  // RX_D0: Received Bit 0
        p.PC5,  // RX_D1: Received Bit 1
        p.PG13, // TX_D0: Transmit Bit 0
        p.PB13, // TX_D1: Transmit Bit 1
        p.PG11, // TX_EN: Transmit Enable
        GenericSMI::new(0),
        mac_addr,
    );

    let config = embassy_net::Config::dhcpv4(Default::default());
    // let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
    //    address: Ipv4Cidr::new(Ipv4Address::new(10, 42, 0, 61), 24),
    //    dns_servers: Vec::new(),
    //    gateway: Some(Ipv4Address::new(10, 42, 0, 1)),
    // });

    // Init network stack
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) =
        embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

    // Launch network task
    spawner.spawn(net_task(runner)).unwrap();

    hprintln!("Waiting for DHCP");

    // Ensure DHCP configuration is up before trying connect
    stack.wait_config_up().await;

    // Then we can use it!
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

        socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

        // You need to start a server on the host machine, for example: `nc -l 8000`
        let remote_endpoint = (Ipv4Address::new(10, 42, 0, 1), 8000);
        hprintln!("connecting...");
        let r = socket.connect(remote_endpoint).await;
        if let Err(e) = r {
            hprintln!("connect error: {:?}", e);
            Timer::after_secs(1).await;
            continue;
        }
        hprintln!("connected!");
        loop {
            let r = socket.write_all(b"Hello\n").await;
            if let Err(e) = r {
                hprintln!("write error: {:?}", e);
                break;
            }
            Timer::after_secs(1).await;
        }
    }

    boot_firmware(&flash)
}
