//! Main

#![no_main]
#![no_std]

use core::num::NonZeroU16;
use core::num::NonZeroU8;

use defmt::todo;
// use embassy_stm32::rng;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::Ipv4Address;
use embassy_net::StackResources;
use embassy_stm32::bind_interrupts;
use embassy_stm32::can;
use embassy_stm32::can::config::DataBitTiming;
use embassy_stm32::can::config::NominalBitTiming;
use embassy_stm32::can::config::{self};
use embassy_stm32::can::frame::CanHeader;
use embassy_stm32::can::RxFdBuf;
use embassy_stm32::can::TxFdBuf;
use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::GenericPhy;
use embassy_stm32::eth::PacketQueue;
use embassy_stm32::eth::{self};
use embassy_stm32::peripherals;
use embassy_stm32::peripherals::*;
use embassy_stm32::rcc;
use embassy_stm32::rng::Rng;
use embassy_stm32::rng::{self};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::PubSubBehavior;
use embassy_sync::pubsub::WaitResult;
use embassy_time::Instant;
use embassy_time::Timer;
use embedded_can::Id;
use embedded_io_async::Write;
use fsm::utils::types::EventChannel;
use fsm::utils::types::EventReceiver;
use fsm::utils::types::EventSender;
use fsm::{Event, FSM};
use main::can::can1;
use main::can::can2;
use main::gs_master;
use main::gs_master::Datapoint;
use main::gs_master::EthernetGsCommsLayerInitializer;
use main::gs_master::GsCommsLayer;
use main::gs_master::GsMaster;
use main::gs_master::GsToPodMessage;
use main::gs_master::PodToGsMessage;
use panic_probe as _;
use rand_core::RngCore as _;
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

fn hlt() -> ! {
    loop {
        cortex_m::asm::wfe();
    }
}

type Device = Ethernet<'static, ETH, GenericPhy>;

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device>) -> ! {
    runner.run().await
}

// Initialize the channel for publishing events to the FSMs.
static EVENT_CHANNEL_FSM: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
static EVENT_CHANNEL_CAN1: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
static EVENT_CHANNEL_CAN2: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();

#[embassy_executor::task]
async fn run_fsm(
    event_receiver: EventReceiver,
    event_sender1: EventSender,
    event_sender2: EventSender,
) {
    let mut fsm = FSM::new(event_receiver, event_sender1, event_sender2).await;
    fsm.run().await;
}

#[embassy_executor::task]
async fn forward_gs_to_fsm(
    mut gsrx: gs_master::RxSubscriber<'static>,
    event_sender: EventSender,
    cantx: can2::CanTxSender<'static>,
) {
    let mut prop_debug_params: u64 = 0;
    let mut prop_test_control_params: u64 = 0;
    loop {
        let msg = gsrx.next_message_pure().await;
        info!("Received message from GS: {:?}", msg);
        let command = msg.command;

        match command {
            // General Commands
            main::config::Command::EmergencyBrake(_) => {
                event_sender
                    .send(fsm::utils::Event::Emergency {
                        emergency_type: fsm::utils::EmergencyType::GeneralEmergency,
                    })
                    .await
            }
            main::config::Command::DefaultCommand(_) => {
                event_sender.send(fsm::utils::Event::NoEvent).await
            }
            main::config::Command::Heartbeat(_) => {

            },
            main::config::Command::FrontendHeartbeat(_) => {

            },
            main::config::Command::EmitEvent(_) => todo!(),

            // HV commands
            main::config::Command::StartHV(_) => {
                event_sender.send(fsm::utils::Event::StartPreCharge).await
            }
            main::config::Command::StopHV(_) => {
                event_sender.send(fsm::utils::Event::Discharge).await
            }

            // Levi commands
            main::config::Command::LevitationOn(_) => {
                event_sender.send(fsm::utils::Event::Levitate).await
            }
            main::config::Command::LevitationOff(_) => {
                event_sender.send(fsm::utils::Event::StopLevitating).await
            }
            main::config::Command::vertical_0_current(_) => todo!(),
            main::config::Command::vert_0_current_reset(_) => todo!(),

            // Propulsion commands
            main::config::Command::PropulsionOn(_) => {
                event_sender.send(fsm::utils::Event::Accelerate).await
            }
            main::config::Command::PropulsionOff(_) => {
                event_sender.send(fsm::utils::Event::Cruise).await
            }
            main::config::Command::SendPropulsionControlWord(value) => {
                send_can2_message(
                    &((value & 0x3FF) as u16).to_le_bytes(),
                    cantx,
                    main::config::Command::SendPropulsionControlWord(value).to_id(),
                ).await;
            }
            main::config::Command::PPControlParams(value) => {
                send_can2_message(
                    &value.to_le_bytes(),
                    cantx,
                    main::config::Command::PPControlParams(value).to_id()
                ).await;
            }
            main::config::Command::PPDebugParams11(value) => {
                prop_debug_params = value;
            }
            main::config::Command::PPDebugParams12(value) => {
                let mut data: [u8; 8] = [0; 8];
                data[0..4].copy_from_slice(&(value as u32).to_le_bytes());
                data[4..8].copy_from_slice(&(prop_debug_params as u32).to_le_bytes());

                send_can2_message(
                    &data,
                    cantx,
                    main::config::Command::PPDebugParams12(value).to_id()
                ).await;
            }
            main::config::Command::PPDebugParams2(value) => {
                send_can2_message(
                    &value.to_le_bytes(),
                    cantx,
                    main::config::Command::PPDebugParams2(value).to_id()
                ).await;
            }
            main::config::Command::PPTestControlParams1(value) => {
                prop_test_control_params = value;
            }
            main::config::Command::PPTestControlParams2(value) => {
                let mut data: [u8; 8] = [0; 8];
                data[0..4].copy_from_slice(&(value as u32).to_le_bytes());
                data[4..8].copy_from_slice(&(prop_test_control_params as u32).to_le_bytes());

                send_can2_message(
                    &data,
                    cantx,
                    main::config::Command::PPTestControlParams2(prop_test_control_params).to_id()
                ).await;
            }

            // Control commands
            main::config::Command::ArmBrakes(_) => {
                event_sender.send(fsm::utils::Event::EnterDemo).await
            }
            main::config::Command::Shutdown(_) => {
                event_sender.send(fsm::utils::Event::ShutDown).await
            }
            main::config::Command::SystemReset(_) => todo!(),

            _ => {
                info!(
                    "Received unknown or uninterpreted command from GS: {:?}",
                    command
                );
            }
        }
    }
}

/// Sends a message on the 2nd CAN bus with the values provided.
///
/// # Params:
/// - `value` The data to be sent
/// - `cantx` The sender object for the 2nd CAN bus
/// - `id` The id of the message
async fn send_can2_message(
    value: &[u8],
    cantx: can2::CanTxSender<'static>,
    id: u16,
) {
    let header = can::frame::Header::new_fd(
        embedded_can::Id::from(
            embedded_can::StandardId::new(
                id,
            ).expect("Invalid ID"),
        ),
        value.len() as u8,
        false,
        true,
    );
    let frame = can::frame::Frame::new(header, value).expect("Invalid frame!");
    cantx.send(can2::CanEnvelope::new_from_frame(frame)).await;
}

#[embassy_executor::task]
async fn forward_fsm_relay_events_to_can1(
    cantx: can1::CanTxSender<'static>,
    mut event_receiver: EventReceiver,
) {
    loop {
        let event = event_receiver.receive().await;
        match event {
            fsm::Event::HighVoltageOnCanRelay => {
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

                cantx.send(can1::CanEnvelope::new_from_frame(frame)).await;
            }

            _ => {}
        }
    }
}

#[embassy_executor::task]
async fn forward_fsm_relay_events_to_can2(
    cantx: can2::CanTxSender<'static>,
    mut event_receiver: EventReceiver,
) {
    loop {
        let event = event_receiver.receive().await;
        match event {
            fsm::Event::HighVoltageOnCanRelay => {
                let header = can::frame::Header::new_fd(
                    embedded_can::Id::from(
                        embedded_can::StandardId::new(0x00000001).expect("Invalid ID"),
                    ),
                    64,
                    false,
                    true,
                );

                let frame = can::frame::Frame::new(header, &[0; 64]).expect("Invalid frame");

                cantx.send(can2::CanEnvelope::new_from_frame(frame)).await;
            }

            _ => {}
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
            Id::Standard(s) => {
                warn!("Received standard (non-extended) CAN ID: {}", s.as_raw());
                continue;
            }
        };

        let fsm_event = main::config::event_for_can_1_id(id);

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
            Id::Extended(e) => todo!("Nuh-uh"),
            Id::Standard(s) => s.as_raw(),
        };

        // TODO: Get config to match correct event
        let fsm_event = main::config::event_for_can_2_id(id as u32);

        // let fsm_event = match (id as u32) {
        //     1 => fsm::Event::ConnectToGS,
        //     2 => fsm::Event::StartSystemCheck,
        //     3 => fsm::Event::SystemCheckSuccess,
        //     4 => fsm::Event::StartPreCharge,
        //     5 => fsm::Event::Activate,
        //     6 => fsm::Event::EnterDemo,
        //     7 => fsm::Event::Levitate,
        //     // 8 => fsm::Event::StopLevitating,
        //     8 => fsm::Event::Accelerate,
        //     9 => fsm::Event::Brake,
        //     10 => fsm::Event::ShutDown,
        //     _ => fsm::Event::NoEvent,
        // };

        if fsm_event != fsm::Event::NoEvent {
            event_sender.send(fsm_event).await;
        }
    }
}

#[embassy_executor::task]
async fn forward_can1_messages_to_gs(
    mut canrx: can1::CanRxSubscriber<'static>,
    mut gstx: gs_master::TxSender<'static>,
) {
    loop {
        let can_frame = canrx.next_message_pure().await;
        let id = match can_frame.id() {
            Id::Extended(extended_id) => extended_id.as_raw(),
            Id::Standard(_) => todo!("Nuh-uh"),
        };

        // info!("Received CAN frame with ID: {}", id);

        let data = can_frame.payload();

        main::config::parse_datapoints_can_1(id, data, |dp| async move {
            gstx.send(PodToGsMessage { dp }).await;
        })
        .await;

        Timer::after_micros(10).await;
    }
}

#[embassy_executor::task]
async fn forward_can2_messages_to_gs(
    mut canrx: can2::CanRxSubscriber<'static>,
    mut gstx: gs_master::TxSender<'static>,
) {
    loop {
        let can_frame = canrx.next_message_pure().await;
        let id = match can_frame.id() {
            Id::Extended(extended_id) => todo!("Nuh-uh"),
            Id::Standard(id) => id.as_raw(),
        };

        // info!("Received CAN frame with ID: {}", id);

        let data = can_frame.payload();

        main::config::parse_datapoints_can_2(id as u32, data, |dp| async move {
            gstx.send(PodToGsMessage { dp }).await;
        })
        .await;

        Timer::after_micros(10).await;
    }
}

#[embassy_executor::task]
async fn gs_heartbeat(mut gstx: gs_master::TxSender<'static>) {
    loop {
        // info!("Sending heartbeat");
        gstx.send(PodToGsMessage {
            dp: Datapoint::new(
                main::config::Datatype::FrontendHeartbeating,
                1,
                embassy_time::Instant::now().as_ticks(),
            ),
        })
        .await;
        Timer::after_millis(100).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    defmt::println!("Hello, world!");

    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
        // config.rcc.hsi = Some(HSIPrescaler::DIV1);
        // config.rcc.csi = true;
        // config.rcc.hsi48 = Some(Default::default()); // needed for RNG
        // config.rcc.pll1 = Some(Pll {
        //     source: PllSource::HSI,
        //     prediv: PllPreDiv::DIV4,
        //     mul: PllMul::MUL50,
        //     divp: Some(PllDiv::DIV2),
        //     divq: None,
        //     divr: None,
        // });
        // config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        // config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        // config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        // config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        // config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        // config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        // config.rcc.voltage_scale = VoltageScale::Scale1;

        //// Config can

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
    let event_channel_can1: &mut EventChannel = EVENT_CHANNEL_CAN1.init(EventChannel::new().into());
    let event_receiver_can1: EventReceiver = event_channel_can1.receiver().into();
    let event_sender_can1: EventSender = event_channel_can1.sender().into();
    let event_channel_can2: &mut EventChannel = EVENT_CHANNEL_CAN2.init(EventChannel::new().into());
    let event_receiver_can2: EventReceiver = event_channel_can2.receiver().into();
    let event_sender_can2: EventSender = event_channel_can2.sender().into();

    info!("Embassy initialized!");

    let can1 = {
        let mut configurator = can::CanConfigurator::new(p.FDCAN1, p.PD0, p.PD1, Irqs);

        let config = configurator
            .config()
            // Configuration for 1Mb/s
            .set_nominal_bit_timing(NominalBitTiming {
                prescaler: NonZeroU16::new(10).unwrap(),
                seg1: NonZeroU8::new(8).unwrap(),
                seg2: NonZeroU8::new(3).unwrap(),
                sync_jump_width: NonZeroU8::new(3).unwrap(),
            })
            // Configuration for 2Mb/s
            .set_data_bit_timing(DataBitTiming {
                transceiver_delay_compensation: true,
                prescaler: NonZeroU16::new(5).unwrap(),
                seg1: NonZeroU8::new(7).unwrap(),
                seg2: NonZeroU8::new(4).unwrap(),
                sync_jump_width: NonZeroU8::new(4).unwrap(),
            })
            .set_tx_buffer_mode(config::TxBufferMode::Priority)
            .set_frame_transmit(config::FrameTransmissionConfig::AllowFdCanAndBRS);

        configurator.set_config(config);

        let mut can = configurator.into_normal_mode();

        can1::CanInterface::new(can, spawner)
    };

    let can2 = {
        let mut configurator = can::CanConfigurator::new(p.FDCAN2, p.PB5, p.PB6, Irqs);

        // let config = configurator
        //     .config()
        //     // Configuration for 1Mb/s
        //     // .set_nominal_bit_timing(NominalBitTiming {
        //     //     prescaler: NonZeroU16::new(15).unwrap(),
        //     //     seg1: NonZeroU8::new(5).unwrap(),
        //     //     seg2: NonZeroU8::new(2).unwrap(),
        //     //     sync_jump_width: NonZeroU8::new(1).unwrap(),
        //     // })
        //     .set_nominal_bit_timing(NominalBitTiming {
        //         prescaler: NonZeroU16::new(15).unwrap(),
        //         seg1: NonZeroU8::new(13).unwrap(),
        //         seg2: NonZeroU8::new(2).unwrap(),
        //         sync_jump_width: NonZeroU8::new(1).unwrap(),
        //     })
        //     // .set_nominal_bit_timing(NominalBitTiming {
        //     //     prescaler: NonZeroU16::new(15).unwrap(),
        //     //     seg1: NonZeroU8::new(13).unwrap(),
        //     //     seg2: NonZeroU8::new(2).unwrap(),
        //     //     sync_jump_width: NonZeroU8::new(1).unwrap(),
        //     // })
        //     // Configuration for 2Mb/s
        //     // .set_data_bit_timing(DataBitTiming {
        //     //     transceiver_delay_compensation: true,
        //     //     prescaler: NonZeroU16::new(12).unwrap(),
        //     //     seg1: NonZeroU8::new(13).unwrap(),
        //     //     seg2: NonZeroU8::new(2).unwrap(),
        //     //     sync_jump_width: NonZeroU8::new(1).unwrap(),
        //     // })
        //     .set_tx_buffer_mode(config::TxBufferMode::Priority)
        //     .set_frame_transmit(config::FrameTransmissionConfig::AllowFdCanAndBRS);
        configurator.set_bitrate(1_000_000);

        // configurator.set_config(config);

        let mut can = configurator.into_normal_mode();

        can2::CanInterface::new(can, spawner)
    };

    info!("CAN Configured");

    spawner
        .spawn(run_fsm(
            event_receiver_fsm,
            event_sender_can1,
            event_sender_can2,
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

    // Generate random seed.
    // let mut rng = Rng::new(p.RNG, Irqs);
    // let mut seed = [0; 8];
    // rng.fill_bytes(&mut seed);
    let seed = 0x123456789ABCDEF_u64;

    let mac_addr = main::config::POD_MAC_ADDRESS;

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
        // main pcb:
        // p.PB12, // TX_D0: Transmit Bit 0
        // nucleo:
        p.PG13, // TX_D0: Transmit Bit 0
        p.PB13, // TX_D1: Transmit Bit 1
        // nucleo:
        p.PG11, // TX_EN: Transmit Enable
        // main pcb:
        // p.PB11,
        GenericPhy::new(0),
        mac_addr,
    );

    let config = embassy_net::Config::dhcpv4(Default::default());
    // let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
    //    address: embassy_net::Ipv4Cidr::new(Ipv4Address::new(169, 254, 42, 80), 16),
    //    dns_servers: heapless::Vec::new(),
    //    gateway: None,
    // });

    let gs_master = GsMaster::new(
        EthernetGsCommsLayerInitializer::new(device, config),
        spawner,
    )
    .await;

    let gsrx = gs_master.subscribe();
    let gstx = gs_master.transmitter();

    unwrap!(spawner.spawn(forward_gs_to_fsm(gsrx, event_sender_gs_to_fsm, can2.new_sender())));

    unwrap!(spawner.spawn(forward_can1_messages_to_gs(
        can1.new_subscriber(),
        gs_master.transmitter()
    )));

    unwrap!(spawner.spawn(forward_can2_messages_to_gs(
        can2.new_subscriber(),
        gs_master.transmitter()
    )));

    unwrap!(spawner.spawn(gs_heartbeat(gs_master.transmitter())));

    loop {
        Timer::after_millis(100).await;
    }

    // loop {
    //     info!("Trying!");

    //     let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
    //     socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

    //     // You need to start a server on the host machine, for example: `nc -l
    // 8000`     let remote_endpoint = (Ipv4Address::new(192, 168, 1, 17),
    // 8000);     let r = socket.connect(remote_endpoint).await;
    //     if let Err(e) = r {
    //         error!("{}", e);
    //         // hprintln!("connect error: {:?}", e);
    //         Timer::after_secs(1).await;
    //         continue;
    //     }
    //     // hprintln!("connected!");

    //     let start_instant = embassy_time::Instant::now();

    //     unwrap!(socket.write_all(&to_write).await);
    //     unwrap!(socket.flush().await);

    //     let end_instant = embassy_time::Instant::now();

    //     let diff = end_instant - start_instant;

    //     info!("Wrote {} bytes in {}us", to_write.len(), diff.as_micros());

    //     socket.close();
    // }

    // let canrx = can.new_subscriber();

    hlt()
}
