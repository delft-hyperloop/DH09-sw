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

use defmt::todo;
use fsm::utils::types::{EventChannel, EventReceiver, EventSender};
use fsm::FSM;

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
static EVENT_CHANNEL: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
static EVENT_RECEIVER: static_cell::StaticCell<EventReceiver> = static_cell::StaticCell::new();
static EVENT_SENDER: static_cell::StaticCell<EventSender> = static_cell::StaticCell::new();

#[embassy_executor::task]
async fn run_fsm(
    event_receiver: &'static EventReceiver,
    event_sender: &'static EventSender,
) {
    let mut fsm = FSM::new(event_sender, event_receiver).await;
    fsm.run().await;
}

static mut read_buffer: RxFdBuf<5> = RxFdBuf::new();
static mut write_buffer: TxFdBuf<1> = TxFdBuf::new();

#[embassy_executor::task]
async fn forward_gs_to_fsm(
    mut gsrx: gs_master::RxSubscriber<'static>,
    event_channel: PriorityEventPubSub,
) {
    loop {
        let msg = gsrx.next_message_pure().await;
        debug!("Received message from GS: {:?}", msg);
        let command = msg.command;

        continue;

        match command {
            main::config::Command::DefaultCommand(_) => todo!(),
            main::config::Command::Heartbeat(_) => todo!(),
            main::config::Command::FrontendHeartbeat(_) => todo!(),
            main::config::Command::LevitationOn(_) => event_channel.add_event(&Event::LevitationOn).await,
            main::config::Command::LevitationOff(_) => event_channel.add_event(&Event::LevitationOff).await,
            main::config::Command::EmergencyBrake(_) => event_channel.add_event(&Event::Emergency).await,
            main::config::Command::StartHV(_) => event_channel.add_event(&Event::HighVoltageOn).await,
            main::config::Command::StopHV(_) => event_channel.add_event(&Event::HighVoltageOff).await,
            main::config::Command::SystemReset(_) => todo!(),
            main::config::Command::ArmBrakes(_) => todo!(),
            main::config::Command::Shutdown(_) => event_channel.add_event(&Event::ShutDown).await,
            main::config::Command::vertical_0_current(_) => todo!(),
            main::config::Command::vert_0_current_reset(_) => todo!(),
            main::config::Command::PropulsionOn(_) => event_channel.add_event(&Event::PropulsionOn).await,
            main::config::Command::PropulsionOff(_) => event_channel.add_event(&Event::PropulsionOff).await,
            main::config::Command::PropulsionStart(_) => event_channel.add_event(&Event::Accelerate {velocity_profile: 0}).await, // TODO
            main::config::Command::SubmitRunConfig(_) => todo!()
        }
    }
}

#[embassy_executor::task]
async fn forward_fsm_relay_events_to_can1(
    cantx: can1::CanTxSender<'static>,
    mut event_channel: PriorityEventPubSub,
) {
    loop {
        let event = event_channel.get_event().await;
        match event {
            fsm::commons::data::Event::HighVoltageOnCanRelay => {
                let header = can::frame::Header::new_fd(
                    embedded_can::Id::from(
                        embedded_can::ExtendedId::new(0x00000001).expect("Invalid ID"),
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
    mut event_channel: EventChannel,
) {
    loop {
        let event = event_channel.receive().await;
        match event {
            fsm::commons::data::Event::HighVoltageOnCanRelay => {
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
    event_channel: EventChannel,
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

        if fsm_event != fsm::utils::Event::NoEvent {
            event_channel.add_event(&fsm_event).await;
        }
    }
}

#[embassy_executor::task]
async fn forward_can2_messages_to_fsm(
    mut canrx: can2::CanRxSubscriber<'static>,
    event_channel: PriorityEventPubSub,
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

        let fsm_event = main::config::event_for_can_2_id(id as u32);

        if fsm_event != fsm::commons::Event::NoEvent {
            event_channel.add_event(&fsm_event).await;
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

        info!("Received CAN frame with ID: {}", id);

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

        info!("Received CAN frame with ID: {}", id);

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

    let event_channel = EVENT_CHANNEL.init(EventChannel::new());
    let emergency_channel = EMERGENCY_CHANNEL.init(EmergencyChannel::new());
    let fsm_state = FSM_STATE.init(Mutex::new(MainStates::SystemCheck));

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

        let config = configurator
            .config()
            // Configuration for 1Mb/s
            // .set_nominal_bit_timing(NominalBitTiming {
            //     prescaler: NonZeroU16::new(15).unwrap(),
            //     seg1: NonZeroU8::new(5).unwrap(),
            //     seg2: NonZeroU8::new(2).unwrap(),
            //     sync_jump_width: NonZeroU8::new(1).unwrap(),
            // })
            .set_nominal_bit_timing(NominalBitTiming {
                prescaler: NonZeroU16::new(15).unwrap(),
                seg1: NonZeroU8::new(13).unwrap(),
                seg2: NonZeroU8::new(2).unwrap(),
                sync_jump_width: NonZeroU8::new(1).unwrap(),
            })
            // .set_nominal_bit_timing(NominalBitTiming {
            //     prescaler: NonZeroU16::new(15).unwrap(),
            //     seg1: NonZeroU8::new(13).unwrap(),
            //     seg2: NonZeroU8::new(2).unwrap(),
            //     sync_jump_width: NonZeroU8::new(1).unwrap(),
            // })
            // Configuration for 2Mb/s
            // .set_data_bit_timing(DataBitTiming {
            //     transceiver_delay_compensation: true,
            //     prescaler: NonZeroU16::new(12).unwrap(),
            //     seg1: NonZeroU8::new(13).unwrap(),
            //     seg2: NonZeroU8::new(2).unwrap(),
            //     sync_jump_width: NonZeroU8::new(1).unwrap(),
            // })
            .set_tx_buffer_mode(config::TxBufferMode::Priority)
            .set_frame_transmit(config::FrameTransmissionConfig::AllowFdCanAndBRS);
        // configurator.set_bitrate(500_000);

        configurator.set_config(config);

        let mut can = configurator.into_normal_mode();

        can2::CanInterface::new(can, spawner)
    };

    info!("CAN Configured");

    spawner
        .spawn(run_fsm(
            spawner,
            event_channel,
            emergency_channel,
            fsm_state,
        ))
        .unwrap();

    let prio = PriorityEventPubSub::new(
        event_channel.publisher().unwrap(),
        event_channel.subscriber().unwrap(),
        emergency_channel.publisher().unwrap(),
        emergency_channel.subscriber().unwrap(),
    );
    unwrap!(spawner.spawn(forward_fsm_relay_events_to_can1(can1.new_sender(), prio)));

    let prio = PriorityEventPubSub::new(
        event_channel.publisher().unwrap(),
        event_channel.subscriber().unwrap(),
        emergency_channel.publisher().unwrap(),
        emergency_channel.subscriber().unwrap(),
    );
    unwrap!(spawner.spawn(forward_can1_messages_to_fsm(can1.new_subscriber(), prio)));

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
        p.PB12, // TX_D0: Transmit Bit 0
        // nucleo:
        // p.PG13, // TX_D0: Transmit Bit 0
        p.PB13, // TX_D1: Transmit Bit 1
        // nucleo:
        // p.PG11, // TX_EN: Transmit Enable
        // main pcb:
        p.PB11,
        GenericPhy::new(0),
        mac_addr,
    );

    let config = embassy_net::Config::dhcpv4(Default::default());
    // let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
    //    address: Ipv4Cidr::new(Ipv4Address::new(10, 42, 0, 61), 24),
    //    dns_servers: Vec::new(),
    //    gateway: Some(Ipv4Address::new(10, 42, 0, 1)),
    // });

    let gs_master = GsMaster::new(
        EthernetGsCommsLayerInitializer::new(device, config),
        spawner,
    )
    .await;

    let gsrx = gs_master.subscribe();
    let gstx = gs_master.transmitter();

    let prio = PriorityEventPubSub::new(
        event_channel.publisher().unwrap(),
        event_channel.subscriber().unwrap(),
        emergency_channel.publisher().unwrap(),
        emergency_channel.subscriber().unwrap(),
    );

    unwrap!(spawner.spawn(forward_gs_to_fsm(gsrx, prio)));

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
