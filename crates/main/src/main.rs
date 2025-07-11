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
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use fsm::FSM;
use lib::EventChannel;
use lib::EventReceiver;
use lib::EventSender;
use main::can as can2;
use main::comms_tasks::check_critical_datapoints;
use main::comms_tasks::forward_can2_messages_to_fsm;
use main::comms_tasks::forward_can2_messages_to_gs;
use main::comms_tasks::forward_fsm_events_to_can2;
use main::comms_tasks::forward_fsm_to_gs;
use main::comms_tasks::forward_gs_to_can2;
use main::comms_tasks::forward_gs_to_fsm;
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

/// fsm priority channel for events
static EVENT_CHANNEL_FSM: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
/// can 2 priority channel for events
static EVENT_CHANNEL_CAN2: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
/// priority channel for events from the fsm to the gs
static EVENT_CHANNEL_GS: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();

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
    event_sender2: EventSender,
    event_sender_gs: EventSender,
    rearm_sdc_pin: Output<'static>,
    sdc_pin: Output<'static>,
) {
    let mut fsm = FSM::new(
        event_receiver,
        event_sender2,
        event_sender_gs,
        rearm_sdc_pin,
        sdc_pin,
    )
    .await;
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

    // The event channel that will be used to transmit events to the FSM.
    let event_channel_fsm: &mut EventChannel = EVENT_CHANNEL_FSM.init(EventChannel::new());
    let event_receiver_fsm: EventReceiver = event_channel_fsm.receiver().into();
    let event_sender_can2_to_fsm: EventSender = event_channel_fsm.sender().into();
    let event_sender_gs_to_fsm: EventSender = event_channel_fsm.sender().into();
    let event_sender_stale_data_checker: EventSender = event_channel_fsm.sender().into();

    // The event channel that will be used to transmit events from the FSM over the
    // CAN bus
    let event_channel_can2: &mut EventChannel = EVENT_CHANNEL_CAN2.init(EventChannel::new());
    let event_receiver_can2: EventReceiver = event_channel_can2.receiver().into();
    let event_sender_can2: EventSender = event_channel_can2.sender().into();

    let event_channel_gs: &mut EventChannel = EVENT_CHANNEL_GS.init(EventChannel::new());
    let event_receiver_fsm_to_gs: EventReceiver = event_channel_gs.receiver().into();
    let event_sender_fsm_to_gs: EventSender = event_channel_gs.sender().into();

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

    // launch the task for the embassy executor to take over
    spawner
        .spawn(run_fsm(
            event_receiver_fsm,
            event_sender_can2,
            event_sender_fsm_to_gs,
            rearm_sdc_pin,
            sdc_pin,
        ))
        .unwrap();

    // tasks for rerouting between communication channels
    unwrap!(spawner.spawn(forward_fsm_events_to_can2(
        can2.new_sender(),
        event_receiver_can2
    )));

    unwrap!(spawner.spawn(forward_can2_messages_to_fsm(
        can2.new_subscriber(),
        event_sender_can2_to_fsm
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
        )
        .await,
    );

    // pass a signal that will get triggered when the first connection is
    // established, so after that we may start checking for stale data (critical
    // data that we didnt receive within a timeout)
    let signal = SIGNAL_CONNECTED.init(Signal::new());

    unwrap!(spawner.spawn(run_gs_master(gs_master, signal)));

    unwrap!(spawner.spawn(forward_gs_to_fsm(
        gs_comms.rx_receiver(),
        event_sender_gs_to_fsm,
    )));

    unwrap!(spawner.spawn(forward_gs_to_can2(
        gs_comms.rx_receiver(),
        gs_comms.tx_publisher(),
        can2.new_sender()
    )));

    unwrap!(spawner.spawn(forward_can2_messages_to_gs(
        can2.new_subscriber(),
        gs_comms.tx_publisher()
    )));

    unwrap!(spawner.spawn(forward_fsm_to_gs(
        gs_comms.tx_publisher(),
        event_receiver_fsm_to_gs
    )));

    unwrap!(spawner.spawn(gs_heartbeat(gs_comms.tx_publisher())));

    unwrap!(spawner.spawn(check_critical_datapoints(
        can2.new_subscriber(),
        event_sender_stale_data_checker,
        gs_comms.tx_publisher(),
        signal
    )));

    // unwrap!(spawner.spawn(log_can2_on_gs(
    //     gs_comms.tx_publisher(),
    //     can2.new_subscriber()
    // )));

    // keep main running, or program exits!
    loop {
        Timer::after_millis(100_000).await;
    }
}
