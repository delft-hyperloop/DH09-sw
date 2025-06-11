//! Main

#![no_main]
#![no_std]

use defmt::todo;
// use embassy_stm32::rng;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::can;
use embassy_stm32::can::frame::Header;
use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::GenericPhy;
use embassy_stm32::eth::PacketQueue;
use embassy_stm32::eth::{self};
use embassy_stm32::gpio::Level;
use embassy_stm32::gpio::Output;
use embassy_stm32::gpio::Speed;
use embassy_stm32::peripherals;
use embassy_stm32::peripherals::*;
use embassy_sync::pubsub::WaitResult;
use embassy_time::Timer;
use embedded_can::Id;
use embedded_can::StandardId;
use fsm::FSM;
use lib::config::Command;
use lib::config::Datatype;
use lib::Datapoint;
use lib::Event;
use lib::EventChannel;
use lib::EventReceiver;
use lib::EventSender;
use main::can::can1;
use main::can::can2;
use main::gs_master;
use main::gs_master::EthernetGsCommsLayerInitializer;
use main::gs_master::GsMaster;
use main::gs_master::PodToGsMessage;
use panic_probe as _;
use static_cell::StaticCell;

bind_interrupts!(
    struct Irqs {
        ETH => eth::InterruptHandler;
        // HASH_RNG => rng::InterruptHandler<peripherals::RNG>;

        // CAN
        FDCAN1_IT0 => can::IT0InterruptHandler<peripherals::FDCAN1>;
        FDCAN1_IT1 => can::IT1InterruptHandler<peripherals::FDCAN1>;

        FDCAN2_IT0 => can::IT0InterruptHandler<peripherals::FDCAN2>;
        FDCAN2_IT1 => can::IT1InterruptHandler<peripherals::FDCAN2>;
    }
);

/// !halt (loop forever)
fn hlt() -> ! {
    loop {
        cortex_m::asm::wfe();
    }
}

#[allow(dead_code)]
/// an ethernet device peripheral, abstract over the specific PHY used
type Device = Ethernet<'static, ETH, GenericPhy>;

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device>) -> ! {
    runner.run().await
}

// Initialize the channel for publishing events to the FSMs.
/// fsm priority channel for events
static EVENT_CHANNEL_FSM: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
/// can 1 priority channel for events
static EVENT_CHANNEL_CAN1: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
/// can 2 priority channel for events
static EVENT_CHANNEL_CAN2: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
/// priority channel for events from the fsm to the gs
static EVENT_CHANNEL_GS: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();

#[embassy_executor::task]
async fn run_fsm(
    event_receiver: EventReceiver,
    event_sender2: EventSender,
    event_sender_gs: EventSender,
) {
    let mut fsm = FSM::new(event_receiver, event_sender2, event_sender_gs).await;
    fsm.run().await;
}

#[embassy_executor::task]
async fn forward_gs_to_fsm(
    mut gsrx: gs_master::RxSubscriber<'static>,
    event_sender: EventSender,
    mut rearm_output: Output<'static>,
) {
    loop {
        let msg = gsrx.next_message_pure().await;
        trace!("Received message from GS: {:?}", msg);
        let command = msg.command;

        match command {
            // General commands
            // TODO: Turn off High Voltage!
            Command::GeneralEmergency(_) => {
                event_sender
                    .send(lib::Event::Emergency {
                        emergency_type: lib::EmergencyType::GeneralEmergency,
                    })
                    .await
            }
            Command::EmergencyBrake(_) => {
                event_sender
                    .send(lib::Event::Emergency {
                        emergency_type: lib::EmergencyType::GeneralEmergency,
                    })
                    .await
            }

            // HV commands
            Command::StartHV(_) => event_sender.send(lib::Event::StartPreCharge).await,
            Command::StopHV(_) => event_sender.send(lib::Event::Discharge).await,

            // Levi commands
            Command::LevitationOn(_) => event_sender.send(lib::Event::Levitate).await,
            Command::LevitationOff(_) => event_sender.send(lib::Event::StopLevitating).await,

            // Propulsion commands
            Command::PropulsionOn(_) => event_sender.send(lib::Event::Accelerate).await,
            Command::PropulsionOff(_) => event_sender.send(lib::Event::Cruise).await,

            // Control commands
            Command::SystemCheck(_) => {
                todo!()
            }
            Command::Shutdown(_) => event_sender.send(lib::Event::ShutDown).await,
            Command::RearmSDC(_) => {
                // Pull pin high
                rearm_output.set_high();

                embassy_time::Timer::after_millis(100).await;
                event_sender.send(lib::Event::EnterDemo).await;

                // Pull pin low
                rearm_output.set_low();
            }

            // Reset commands
            Command::SystemReset(_) => event_sender.send(lib::Event::ResetFSM).await,
            Command::ResetSenseCon(_) => event_sender.send(lib::Event::ResetFSM).await,
            Command::ResetLevitation(_) => {
                todo!()
            }
            Command::ResetPowertrain(_) => {
                todo!()
            }
            Command::ResetPropulsion(_) => {
                todo!()
            }

            _ => {}
        }
    }
}

#[embassy_executor::task]
async fn forward_gs_to_can1(
    mut gsrx: gs_master::RxSubscriber<'static>,
    cantx: can1::CanTxSender<'static>,
) {
    loop {
        let msg = gsrx.next_message_pure().await;
        info!("Received message from GS: {:?}", msg);
        let command = msg.command;

        lib::config::gs_to_can1(command, |frame| cantx.send(frame)).await;
    }
}

#[embassy_executor::task]
async fn forward_gs_to_can2(
    mut gsrx: gs_master::RxSubscriber<'static>,
    cantx: can2::CanTxSender<'static>,
) {
    loop {
        let msg = gsrx.next_message_pure().await;
        trace!("Received message from GS: {:?}", msg);
        let command = msg.command;

        lib::config::gs_to_can2(command, |frame| cantx.send(frame)).await;
    }
}

#[embassy_executor::task]
async fn forward_fsm_relay_events_to_can1(
    cantx: can1::CanTxSender<'static>,
    event_receiver: EventReceiver,
) {
    loop {
        let event = event_receiver.receive().await;
        if event == fsm::Event::HighVoltageOnCanRelay {
            let header = can::frame::Header::new_fd(
                embedded_can::Id::from(
                    embedded_can::StandardId::new(
                        // main::config::Command::HighVoltageOnCanRelay(0).to_id() as u32,
                        10,
                    )
                    .expect("Invalid ID"),
                ),
                64,
                false,
                true,
            );

            let frame = can::frame::FdFrame::new(header, &[0; 64]).expect("Invalid frame");

            cantx
                .send(lib::can::can1::CanEnvelope::new_from_frame(frame))
                .await;
        }
    }
}

#[embassy_executor::task]
async fn forward_fsm_relay_events_to_can2(
    cantx: can2::CanTxSender<'static>,
    event_receiver: EventReceiver,
) {
    loop {
        let event = event_receiver.receive().await;
        if let fsm::Event::FSMTransition(state_number) = event {
            cantx
                .send(lib::can::can2::CanEnvelope::new_with_id(
                    Command::FSMUpdate(0).to_id(),
                    &[state_number],
                ))
                .await
        }
    }
}

#[embassy_executor::task]
async fn forward_can1_messages_to_fsm(
    mut canrx: can1::CanRxSubscriber<'static>,
    event_sender: EventSender,
) {
    loop {
        let msg = canrx.next_message().await;

        let envelope = match msg {
            WaitResult::Message(envelope) => envelope,
            WaitResult::Lagged(i) => {
                warn!("Lagged {} messages", i);
                continue;
            }
        };

        let id = match envelope.id() {
            Id::Extended(e) => e.as_raw(),
            Id::Standard(s) => s.as_raw() as u32,
        };

        info!("Received CAN frame with ID: {}", id);
        let fsm_event = lib::config::event_for_can_1_id(id);

        if fsm_event != fsm::Event::NoEvent {
            event_sender.send(fsm_event).await;
        }
    }
}

#[embassy_executor::task]
async fn forward_can2_messages_to_fsm(
    mut canrx: can2::CanRxSubscriber<'static>,
    event_sender: EventSender,
) {
    loop {
        let msg = canrx.next_message().await;

        let envelope = match msg {
            WaitResult::Message(envelope) => envelope,
            WaitResult::Lagged(i) => {
                warn!("Lagged {} messages", i);
                continue;
            }
        };

        let id = match envelope.id() {
            Id::Extended(_e) => todo!("Nuh-uh"),
            Id::Standard(s) => s.as_raw(),
        };

        // TODO: Get config to match correct event
        let fsm_event = lib::config::event_for_can_2_id(id as u32);

        if fsm_event != fsm::Event::NoEvent {
            event_sender.send(fsm_event).await;
        }
    }
}

#[embassy_executor::task]
async fn forward_can1_messages_to_gs(
    mut canrx: can1::CanRxSubscriber<'static>,
    gstx: gs_master::TxSender<'static>,
) {
    loop {
        let can_frame = canrx.next_message_pure().await;
        let id = match can_frame.id() {
            Id::Standard(s) => s.as_raw() as u32,
            Id::Extended(e) => {
                warn!("Received extended CAN ID on can1->gs: {}", e.as_raw());
                continue;
            }
        };

        // info!("Received CAN frame with ID: {}", id);

        let data = can_frame.payload();
        // id = id as u32;
        lib::config::parse_datapoints_can_1(id, data, |dp| async move {
            gstx.send(PodToGsMessage { dp }).await;
        })
        .await;

        // Timer::after_micros(10).await;
    }
}

#[embassy_executor::task]
async fn forward_can2_messages_to_gs(
    mut canrx: can2::CanRxSubscriber<'static>,
    gstx: gs_master::TxSender<'static>,
) {
    loop {
        let can_frame = canrx.next_message_pure().await;
        let id = match can_frame.id() {
            Id::Extended(_extended_id) => todo!("Nuh-uh"),
            Id::Standard(id) => id.as_raw(),
        };

        // info!("Received CAN frame with ID: {}", id);

        let data = can_frame.payload();

        lib::config::parse_datapoints_can_2(id as u32, data, |dp| async move {
            gstx.send(PodToGsMessage { dp }).await;
        })
        .await;

        // Timer::after_micros(10).await;
    }
}

/// Forwards all CAN messages to the groundstation for logging
#[embassy_executor::task]
async fn log_can2_on_gs(
    gstx: gs_master::TxSender<'static>,
    mut canrx: can2::CanRxSubscriber<'static>,
) {
    loop {
        let can_frame = canrx.next_message_pure().await;
        let id = match can_frame.id() {
            Id::Standard(s) => s.as_raw() as u32,
            Id::Extended(e) => {
                warn!("Received extended CAN ID on can1->gs: {}", e.as_raw());
                continue;
            }
        };

        gstx.send(PodToGsMessage {
            dp: Datapoint::new(
                Datatype::CANLog,
                u64::from(id),
                embassy_time::Instant::now().as_ticks(),
            ),
        })
        .await;
        // Timer::after_millis(50).await;
    }
}

#[embassy_executor::task]
async fn gs_heartbeat(gstx: gs_master::TxSender<'static>) {
    let mut value = 1;
    loop {
        // info!("Sending heartbeat");
        gstx.send(PodToGsMessage {
            dp: Datapoint::new(
                Datatype::FrontendHeartbeating,
                value,
                embassy_time::Instant::now().as_ticks(),
            ),
        })
        .await;
        value = !value;
        Timer::after_millis(100).await;
    }
}

/// Only used for testing, should not be run in the final version
#[embassy_executor::task]
async fn send_random_msg_continuously(can_tx: can2::CanTxSender<'static>) {
    loop {
        let header = Header::new(Id::from(StandardId::new(8u16).unwrap()), 8, false);

        let frame = can::frame::Frame::new(header, &[1u8; 8]).expect("Invalid frame");

        can_tx
            .send(lib::can::can2::CanEnvelope::new_from_frame(frame))
            .await;
        info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>SENDING");

        Timer::after_millis(100).await;
    }
}

#[embassy_executor::task]
async fn forward_fsm_to_gs(gs_tx: gs_master::TxSender<'static>, event_receiver: EventReceiver) {
    loop {
        let event = event_receiver.receive().await;
        match event {
            Event::FSMTransition(transitioned_state) => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::FSMState,
                            transitioned_state as u64,
                            embassy_time::Instant::now().as_ticks(),
                        ),
                    })
                    .await
            }
            Event::TransitionFail(other_state) => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::FSMTransitionFail,
                            other_state as u64,
                            embassy_time::Instant::now().as_ticks(),
                        ),
                    })
                    .await
            }
            Event::Emergency { emergency_type } => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::Emergency,
                            (emergency_type as i32 + 1) as u64,
                            embassy_time::Instant::now().as_ticks(),
                        ),
                    })
                    .await
            }
            _ => {}
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    defmt::println!("Hello, world!");

    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc;

        // Config can
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
        //
    }
    let p = embassy_stm32::init(config);

    // The event channel that will be used to transmit events to the FSM.
    let event_channel_fsm: &mut EventChannel = EVENT_CHANNEL_FSM.init(EventChannel::new());
    let event_receiver_fsm: EventReceiver = event_channel_fsm.receiver().into();
    let event_sender_can1_to_fsm: EventSender = event_channel_fsm.sender().into();
    let event_sender_can2_to_fsm: EventSender = event_channel_fsm.sender().into();
    let event_sender_gs_to_fsm: EventSender = event_channel_fsm.sender().into();

    // The event channel that will be used to transmit events from the FSM over the
    // CAN bus
    let event_channel_can1: &mut EventChannel = EVENT_CHANNEL_CAN1.init(EventChannel::new());
    let event_receiver_can1: EventReceiver = event_channel_can1.receiver().into();
    let event_channel_can2: &mut EventChannel = EVENT_CHANNEL_CAN2.init(EventChannel::new());
    let event_receiver_can2: EventReceiver = event_channel_can2.receiver().into();
    let event_sender_can2: EventSender = event_channel_can2.sender().into();

    let event_channel_gs: &mut EventChannel = EVENT_CHANNEL_GS.init(EventChannel::new());
    let event_receiver_fsm_to_gs: EventReceiver = event_channel_gs.receiver().into();
    let event_sender_fsm_to_gs: EventSender = event_channel_gs.sender().into();

    info!("Embassy initialized!");

    let can1 = {
        let mut configurator = can::CanConfigurator::new(p.FDCAN2, p.PB5, p.PB6, Irqs);

        configurator.set_bitrate(1_000_000);
        let can = configurator.into_normal_mode();

        can1::CanInterface::new(can, spawner)
    };

    let can2 = {
        let mut configurator = can::CanConfigurator::new(p.FDCAN1, p.PB8, p.PB9, Irqs);

        configurator.set_bitrate(1_000_000);
        let can = configurator.into_normal_mode();

        can2::CanInterface::new(can, spawner)
    };

    info!("CAN Configured");
    defmt::println!("CAN Configured");
    spawner
        .spawn(run_fsm(
            event_receiver_fsm,
            event_sender_can2,
            event_sender_fsm_to_gs,
        ))
        .unwrap();

    unwrap!(spawner.spawn(forward_fsm_relay_events_to_can1(
        can1.new_sender(),
        event_receiver_can1
    )));

    unwrap!(spawner.spawn(forward_can1_messages_to_fsm(
        can1.new_subscriber(),
        event_sender_can1_to_fsm
    )));

    unwrap!(spawner.spawn(forward_fsm_relay_events_to_can2(
        can2.new_sender(),
        event_receiver_can2
    )));

    unwrap!(spawner.spawn(forward_can2_messages_to_fsm(
        can2.new_subscriber(),
        event_sender_can2_to_fsm
    )));

    info!("FSM started!");

    let mac_addr = lib::config::POD_MAC_ADDRESS;

    static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
    // warning: Not all STM32H7 devices have the exact same pins here
    // for STM32H747XIH, replace p.PB13 for PG12
    let device = Ethernet::new(
        PACKETS.init(PacketQueue::<4, 4>::new()),
        p.ETH,
        Irqs,
        p.PA1, // ref_clk
        p.PA2, // mdio
        p.PC1, // eth_mdc
        p.PA7, // CRS_DV: Carrier Sense
        p.PC4, // RX_D0: Received Bit 0
        p.PC5, // RX_D1: Received Bit 1
        //choose one:
        p.PB12, // FOR MPCB (TX_D0: Transmit Bit 0)
        // p.PG13, // FOR NUCLEO (TX_D0: Transmit Bit 0)
        p.PB13, // TX_D1: Transmit Bit 1
        //choose one:
        p.PB11, //FOR MPCB (TX_EN: Transmit Enable)
        // p.PG11, // FOR NUCLEO (TX_EN: Transmit Enable)
        GenericPhy::new(0),
        mac_addr,
    );

    let config = embassy_net::Config::dhcpv4(Default::default());

    let gs_master = GsMaster::new(
        EthernetGsCommsLayerInitializer::new(device, config),
        spawner,
    )
    .await;

    let rearm_sdc = Output::new(p.PA10, Level::Low, Speed::Medium);

    unwrap!(spawner.spawn(forward_gs_to_fsm(
        gs_master.subscribe(),
        event_sender_gs_to_fsm,
        rearm_sdc,
    )));

    unwrap!(spawner.spawn(forward_gs_to_can2(gs_master.subscribe(), can2.new_sender())));

    unwrap!(spawner.spawn(forward_can1_messages_to_gs(
        can1.new_subscriber(),
        gs_master.transmitter()
    )));

    unwrap!(spawner.spawn(forward_can2_messages_to_gs(
        can2.new_subscriber(),
        gs_master.transmitter()
    )));

    unwrap!(spawner.spawn(forward_fsm_to_gs(
        gs_master.transmitter(),
        event_receiver_fsm_to_gs
    )));

    unwrap!(spawner.spawn(gs_heartbeat(gs_master.transmitter())));

    unwrap!(spawner.spawn(log_can2_on_gs(
        gs_master.transmitter(),
        can2.new_subscriber()
    )));

    // unwrap!(spawner.spawn(send_random_msg_continuously(can2.new_sender())));

    loop {
        Timer::after_millis(100).await;
    }

    #[allow(unreachable_code)]
    hlt()
}
