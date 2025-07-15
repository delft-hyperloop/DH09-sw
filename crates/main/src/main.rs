//! Main

#![no_main]
#![no_std]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::can;
use embassy_stm32::eth::{self};
use embassy_stm32::gpio::Level;
use embassy_stm32::gpio::Output;
use embassy_stm32::gpio::Speed;
use embassy_stm32::peripherals;
use embassy_stm32::wdg::IndependentWatchdog;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use fsm::FSM;
use lib::EventChannel;
use lib::EventReceiver;
use lib::EventSender;
use main::can as can2;
use main::comms_tasks::check_critical_datapoints;
use main::comms_tasks::forward_can_datapoints;
use main::comms_tasks::forward_fsm_events;
use main::comms_tasks::forward_gs_commands;
use main::comms_tasks::gs_heartbeat;
use main::ethernet::logic::GsMaster;
use main::ethernet::types::EthPeripherals;
use main::ethernet::types::GsComms;
#[cfg(debug_assertions)]
use panic_probe as _;
use static_cell::StaticCell;
#[cfg(not(debug_assertions))]
mod panic_handler; // trigger emergency braking and reboot in release mode

// bind interrupt service routines to the hardware-triggered interrupts of
// different peripherals
bind_interrupts!(
    struct Irqs {
        ETH => eth::InterruptHandler;

        // CAN
        FDCAN1_IT0 => can::IT0InterruptHandler<peripherals::FDCAN1>;
        FDCAN1_IT1 => can::IT1InterruptHandler<peripherals::FDCAN1>;

        FDCAN2_IT0 => can::IT0InterruptHandler<peripherals::FDCAN2>;
        FDCAN2_IT1 => can::IT1InterruptHandler<peripherals::FDCAN2>;
    }
);

/// Priority channel for sending events to the FSM
static EVENT_CHANNEL_IN: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
/// Priority channel for sending events from the FSM to CAN and GS (interpreted
/// to datapoints/commands)
static EVENT_CHANNEL_OUT: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();

/// struct that runs the ethernet stack for connecting to the ground station
static GS_MASTER: StaticCell<GsMaster> = StaticCell::new();
/// struct for the channels used for communicating with the GsMaster
static GS_COMMS: StaticCell<GsComms> = StaticCell::new();
/// a signal fired when the ground station is connected for the first time
static SIGNAL_CONNECTED: StaticCell<Signal<NoopRawMutex, bool>> = StaticCell::new();

/// infinite-looping task for actually running the FSM defined in /crates/fsm
#[embassy_executor::task]
async fn run_fsm(
    event_receiver: EventReceiver,
    event_sender: EventSender,
    rearm_sdc_pin: Output<'static>,
    sdc_pin: Output<'static>,
) -> ! {
    let mut fsm = FSM::new(event_receiver, event_sender, rearm_sdc_pin, sdc_pin).await;
    fsm.run().await;
}

/// task responsible for running ethernet/tcp/groundstation communication.
///
/// when a connection has been established for the first time, a signal is
/// fired, indicating that the pod control is operational
#[embassy_executor::task]
async fn run_gs_master(
    gs_master: &'static mut GsMaster,
    signal_connected: &'static Signal<NoopRawMutex, bool>,
) -> ! {
    gs_master.run_net_fsm(signal_connected).await;
}

/// actual entry point of the program, and the first task picked up by the
/// executor.
#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    defmt::println!("Hello, world!");

    // configure embassy's Peripherals according to the hardware specifications of
    // the PCB this code is to be ran on. These configurations are for DH09
    // custom main pcb, which is meant to be equivalent to a STM Nucleo H743ZI2
    // / H53ZI2 (chip stm32h743zit6u)
    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc;

        // configure the high-speed-internal clock (hsi) phase-locked-loop (pll) and the
        // scalers, in order to provide an appropriate signal for CAN.
        config.rcc.hsi = Some(rcc::HSIPrescaler::DIV1);
        config.rcc.pll1 = Some(rcc::Pll {
            source: rcc::PllSource::HSI,
            prediv: rcc::PllPreDiv::DIV4,  // 64Mhz -> 16MHz
            mul: rcc::PllMul::MUL60,       // 16Mhz -> 960MHz,
            divp: Some(rcc::PllDiv::DIV2), // 960MHz -> 480MHz
            divq: Some(rcc::PllDiv::DIV8), // 960MHz -> 120MHz
            divr: None,
        });
        config.rcc.sys = rcc::Sysclk::PLL1_P; // 480MHz
        config.rcc.ahb_pre = rcc::AHBPrescaler::DIV2; // 240MHz to peripherals

        // Bump down peripheral clocks to 120MHz, which seems like the typical max
        // interface frequency and is mandated by Embassy
        config.rcc.apb1_pre = rcc::APBPrescaler::DIV2;
        config.rcc.apb2_pre = rcc::APBPrescaler::DIV2;
        config.rcc.apb3_pre = rcc::APBPrescaler::DIV2;
        config.rcc.apb4_pre = rcc::APBPrescaler::DIV2;

        // Voltage scaling 0 to support this
        config.rcc.voltage_scale = rcc::VoltageScale::Scale0;

        // 120MHz, must be equal to or less than APB1 bus
        config.rcc.mux.fdcansel = rcc::mux::Fdcansel::PLL1_Q;
    }
    let p = embassy_stm32::init(config);

    info!("Embassy initialized!");

    let can2 = {
        let mut configurator = can::CanConfigurator::new(p.FDCAN1, p.PB8, p.PB9, Irqs);

        configurator.set_bitrate(1_000_000);
        let can = configurator.into_normal_mode();

        can2::CanInterface::new(can, spawner)
    };

    info!("CAN Configured");
    defmt::println!("CAN Configured");

    // Used to test CAN
    // spawner.spawn(send_random_msg_continuously(
    //     can2.new_sender(),
    // )).unwrap();

    // SDC = ShutDown Circuit. Pin PB0 triggers the brakes and shuts off high
    // voltage, pin PA10 rearms the system.
    let rearm_sdc_pin = Output::new(p.PA10, Level::Low, Speed::Medium);
    let mut sdc_pin = Output::new(p.PB0, Level::High, Speed::Medium);
    sdc_pin.set_high();

    // Send events to the fsm
    let event_channel_in_fsm = EVENT_CHANNEL_IN.init(EventChannel::new());
    // Send events from the FSM to the task that translates them to gs datapoints or
    // CAN commands
    let event_channel_out_fsm = EVENT_CHANNEL_OUT.init(EventChannel::new());

    // launch the task for the embassy executor to take over
    unwrap!(spawner.spawn(run_fsm(
        event_channel_in_fsm.receiver().into(),
        event_channel_out_fsm.sender().into(),
        rearm_sdc_pin,
        sdc_pin,
    )));

    info!("FSM started!");

    let gs_comms = GS_COMMS.init(GsComms::new());
    let gs_tx_receiver = gs_comms.tx_receiver();
    let gs_tx_transmitter = gs_comms.tx_publisher();
    let gs_rx_transmitter = gs_comms.rx_publisher();

    // the ethernet task gets ownership of all the ethernet peripherals (incl all
    // pins for talking to the PHY) so no other part of the code can use them.
    let eth_peripherals = EthPeripherals {
        eth: p.ETH,
        pa1: p.PA1,
        pa2: p.PA2,
        pc1: p.PC1,
        pa7: p.PA7,
        pc4: p.PC4,
        pc5: p.PC5,
        pb12: p.PB12,
        pg13: p.PG13,
        pb13: p.PB13,
        pb11: p.PB11,
        pg11: p.PG11,
    };

    // all the eth/tcp/gs communication stuff is held within the static cell of
    // GS_MASTER
    let gs_master = GS_MASTER.init(
        GsMaster::init(
            eth_peripherals,
            spawner,
            Irqs,
            gs_tx_receiver,
            gs_rx_transmitter,
            gs_tx_transmitter,
            event_channel_in_fsm.sender().into(),
        )
        .await,
    );

    // pass a signal that will get triggered when the first connection is
    // established, so after that we may start checking for stale data (critical
    // data that we didnt receive within a timeout)
    let signal = SIGNAL_CONNECTED.init(Signal::new());

    unwrap!(spawner.spawn(run_gs_master(gs_master, signal)));

    unwrap!(spawner.spawn(forward_can_datapoints(
        gs_comms.tx_publisher(),
        event_channel_in_fsm.sender().into(),
        can2.new_subscriber(),
    )));
    unwrap!(spawner.spawn(forward_fsm_events(
        gs_comms.tx_publisher(),
        can2.new_sender(),
        event_channel_out_fsm.receiver().into(),
    )));
    unwrap!(spawner.spawn(forward_gs_commands(
        gs_comms.rx_receiver(),
        event_channel_in_fsm.sender().into(),
        can2.new_sender(),
    )));

    unwrap!(spawner.spawn(gs_heartbeat(gs_comms.tx_publisher())));

    unwrap!(spawner.spawn(check_critical_datapoints(
        can2.new_subscriber(),
        event_channel_in_fsm.sender().into(),
        gs_comms.tx_publisher(),
        signal
    )));

    // unwrap!(spawner.spawn(log_can2_on_gs(
    //     gs_comms.tx_publisher(),
    //     can2.new_subscriber()
    // )));

    #[cfg(debug_assertions)]
    {
        use cortex_m::peripheral::DWT;
        use embassy_time::Instant;

        let measure_start = Instant::now();
        unsafe {
            let mut p = cortex_m::Peripherals::steal();
            p.DCB.enable_trace();
            p.DWT.enable_cycle_counter();
        }

        let (mut prev, mut next) = (0, 0);
        next = DWT::cycle_count();
        loop {
            prev = next;
            next = DWT::cycle_count();
            defmt::warn!("total cycles: {}->{}", prev, next);
            defmt::warn!("delta={}", next - prev);
            defmt::warn!("total sleep: {}", DWT::sleep_count());
            defmt::warn!("total elapsed: {}μs", measure_start.elapsed().as_micros());
            Timer::after_millis(1000).await;
        }
    }

    // Wait to connect to the ground station first, and then unleash the watchdog"
    signal.wait().await;
    let mut puppy = IndependentWatchdog::new(p.IWDG1, 2_500_000);
    #[cfg(not(debug_assertions))]
    puppy.unleash();
    loop {
        Timer::after_millis(20).await;
        puppy.pet();
    }
}
