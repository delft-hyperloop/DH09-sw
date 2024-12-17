#![no_std]
#![no_main]


// Import HAL for STM32H7xx
use fdcan::{
    config::{DataBitTiming, FrameTransmissionConfig, NominalBitTiming},
    filter::{ExtendedFilter, ExtendedFilterSlot, StandardFilter, StandardFilterSlot},
    frame::{FrameFormat, TxFrameHeader},
    id::{ExtendedId, StandardId},
    ConfigMode, FdCan, NormalOperationMode,
};
use stm32h7xx_hal::{can::Can, pac::hrtim_master::micr};
use stm32h7xx_hal::gpio::Speed;
use stm32h7xx_hal::prelude::_embedded_hal_serial_Read;
use stm32h7xx_hal::prelude::*;
use stm32h7xx_hal::timer;
use stm32h7xx_hal::nb::block;
use stm32h7xx_hal::rcc::{self, rec};
use stm32h7xx_hal::serial::{Rx, Serial, Tx};
use stm32h7xx_hal::time::Hertz;
use stm32h7xx_hal::{delay::Delay, pac, prelude::*};
use core::{num::{NonZeroU16, NonZeroU8}};
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};
use pac::interrupt;

use core::{
    cell::RefCell,
    sync::atomic::{AtomicU32, Ordering},
};

use cortex_m::{asm, interrupt::Mutex};

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;


static OVERFLOWS: AtomicU32 = AtomicU32::new(0);
static TIMER: Mutex<RefCell<Option<timer::Timer<pac::TIM2>>>> =
    Mutex::new(RefCell::new(None));

#[entry]
unsafe fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();
    dp.RCC.ahb1enr.write(|w| w.dma1en().set_bit());

    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwr = pwr.vos0(&dp.SYSCFG);
    let pwrcfg = pwr.freeze();
    

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    //let rcc = rcc.sys_ck(400.mhz()).use_hse(8.mhz()).bypass_hse();
    let ccdr = rcc
        .sys_ck(Hertz::MHz(480))
        .pll1_strategy(rcc::PllConfigStrategy::Iterative)
        .pll1_q_ck(Hertz::MHz(120))
        .freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOD peripheral
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);

    assert_eq!(ccdr.clocks.pll1_q_ck().unwrap().raw(), 120_000_000);
    let fdcan_prec = ccdr
        .peripheral
        .FDCAN
        .kernel_clk_mux(rec::FdcanClkSel::Pll1Q);

    let mut timer =
        dp.TIM2
            .tick_timer(1.MHz(), ccdr.peripheral.TIM2, &ccdr.clocks);

    timer.listen(timer::Event::TimeOut);

    cortex_m::interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(timer));
    });

    unsafe {
        cp.NVIC.set_priority(interrupt::TIM2, 1);
        pac::NVIC::unmask(interrupt::TIM2);
    }

    let btr = NominalBitTiming {
        prescaler: NonZeroU16::new(5).unwrap(),
        seg1: NonZeroU8::new(13 + 5).unwrap(),
        seg2: NonZeroU8::new(5).unwrap(),
        sync_jump_width: NonZeroU8::new(4).unwrap(),
    };

    let data_bit_timing = DataBitTiming {
        prescaler: NonZeroU8::new(5).unwrap(),
        seg1: NonZeroU8::new(5).unwrap(),
        seg2: NonZeroU8::new(6).unwrap(),
        sync_jump_width: NonZeroU8::new(4).unwrap(),
        transceiver_delay_compensation: true,
    };

    // Setup fdcan_tq_ck = 120MHz
    let mut can = {
        // Acquire the GPIOD peripheral
        let rx = gpiod.pd0.into_alternate().speed(Speed::VeryHigh);
        let tx = gpiod.pd1.into_alternate().speed(Speed::VeryHigh);
        dp.FDCAN1.fdcan(tx, rx, fdcan_prec)
    };

    can.set_protocol_exception_handling(false);
    can.set_frame_transmit(FrameTransmissionConfig::AllowFdCanAndBRS);
    can.set_nominal_bit_timing(btr);
    can.set_data_bit_timing(data_bit_timing);

    // can.set_standard_filter(
    //     StandardFilterSlot::_0,
    //     StandardFilter::accept_all_into_fifo0(),
    // );

    can.set_extended_filter(
        ExtendedFilterSlot::_0,
        ExtendedFilter::accept_all_into_fifo0(),
    );

    let mut can = can.into_normal();
    
    let mut message: [u8; 64] = [0xFF; 64]; // Sufficient buffer for CAN FD Frame
    let mut total_bytes = 0;
    let mut received = 0;

    let header = TxFrameHeader {
        len: 64,
        id: ExtendedId::new(0x1).unwrap().into(),
        frame_format: FrameFormat::Fdcan,
        bit_rate_switching: true,
        marker: None,
    };

    // loop {
    //     block!(can.transmit(header, &message)).unwrap();
    // }

    let timestamp_start = timestamp();
    for _ in 0..10000 {
        let frame = block!(can.receive0(message.as_mut_slice())).unwrap().unwrap();
        total_bytes += frame.len as u64;
        received += 1;
    }
    let microseconds = timestamp() - timestamp_start;

    hprintln!("Datarate: {}kb/s", (total_bytes * 8 * 1000)/(microseconds));
    hprintln!("Received: {} messages in {}ms", received, microseconds / 1000);

    loop {}
}

#[interrupt]
fn TIM2() {
    OVERFLOWS.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    cortex_m::interrupt::free(|cs| {
        let mut rc = TIMER.borrow(cs).borrow_mut();
        let timer = rc.as_mut().unwrap();
        timer.clear_irq();
    })
}

/// Returns the 64-bit number of microseconds since startup
pub fn timestamp() -> u64 {
    let overflows = OVERFLOWS.load(Ordering::SeqCst) as u64;
    let ctr = cortex_m::interrupt::free(|cs| {
        let rc = TIMER.borrow(cs).borrow();
        let timer = rc.as_ref().unwrap();
        timer.counter() as u64
    });
    (overflows << 32) + ctr
}