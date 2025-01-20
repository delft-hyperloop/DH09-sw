//! Main

#![no_main]
#![no_std]

use core::num::NonZeroU16;
use core::num::NonZeroU8;

// use embassy_stm32::rng;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::Ipv4Address;
use embassy_net::StackResources;
use embassy_stm32::bind_interrupts;
use embassy_stm32::can::frame::CanHeader;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::WaitResult;
use embassy_time::Instant;
use embassy_time::Timer;

use embassy_stm32::can;
use embassy_stm32::can::config::DataBitTiming;
use embassy_stm32::can::config::NominalBitTiming;
use embassy_stm32::can::config::{self};
use embassy_stm32::can::RxFdBuf;
use embassy_stm32::can::TxFdBuf;
use embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::PacketQueue;
use embassy_stm32::eth::{self};
use embassy_stm32::peripherals;
use embassy_stm32::peripherals::*;
use embassy_stm32::rcc;
use embassy_stm32::rng::Rng;
use embassy_stm32::rng::{self};
use embassy_sync::pubsub::PubSubBehavior;
use embedded_can::Id;
use embedded_io_async::Write;
use fsm::commons::data::PriorityEventPubSub;
use fsm::commons::traits::Runner;
use fsm::commons::EmergencyChannel;
use fsm::commons::EventChannel;
use fsm::{MainFSM, MainStates};
use main::can::CanEnvelope;
use main::can::CanInterface;
use main::can::CanRxSubscriber;
use main::can::CanTxSender;
use main::gs_master;
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
        HASH_RNG => rng::InterruptHandler<peripherals::RNG>;

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

type Device = Ethernet<'static, ETH, GenericSMI>;

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device>) -> ! {
    runner.run().await
}

// Initialize the channel for publishing events to the FSMs.
static EVENT_CHANNEL: static_cell::StaticCell<EventChannel> = static_cell::StaticCell::new();
static EMERGENCY_CHANNEL: static_cell::StaticCell<EmergencyChannel> =
    static_cell::StaticCell::new();
static FSM_STATE: static_cell::StaticCell<Mutex<NoopRawMutex, MainStates>> = static_cell::StaticCell::new();

#[embassy_executor::task]
async fn run_fsm(
    spawner: Spawner,
    event_channel: &'static EventChannel,
    emergency_channel: &'static EmergencyChannel,
    state: &'static Mutex<NoopRawMutex, MainStates>,
) {
    let mut main_fsm = MainFSM::new(spawner, state, event_channel, emergency_channel).await;
    main_fsm.run().await;
}

static mut read_buffer: RxFdBuf<5> = RxFdBuf::new();
static mut write_buffer: TxFdBuf<1> = TxFdBuf::new();

#[embassy_executor::task]
async fn forward_gs_to_fsm(mut gsrx: gs_master::RxSubscriber<'static>, event_channel: PriorityEventPubSub) {
    loop {
        let msg = gsrx.next_message_pure().await;
        // info!("Received message from GS: {:?}", msg);
        let fsm_event = msg.fsm_event;

        if fsm_event != fsm::commons::Event::NoEvent {
            event_channel.add_event(&fsm_event).await;
        }
    }
}

#[embassy_executor::task]
async fn forward_fsm_relay_events_to_can(cantx: CanTxSender<'static>, mut event_channel: PriorityEventPubSub) {
    loop {
        let event = event_channel.get_event().await;
        match event {
            fsm::commons::data::Event::HighVoltageOnCanRelay => {
                let header = can::frame::Header::new_fd(
                    can::frame::Id::try_from(0x00000001 as u32).expect("Invalid ID"),
                    64,
                    false,
                    true,
                );
            
                let frame = can::frame::FdFrame::new(header, &[0; 64]).expect("Invalid frame");

                cantx.send(CanEnvelope::new_from_frame(frame)).await;
            }

            _ => {}
        }
    }
}

#[embassy_executor::task]
async fn forward_can_messages_to_fsm(mut canrx: CanRxSubscriber<'static>, event_channel: PriorityEventPubSub) {
    loop {
        let msg = canrx.next_message().await;

        let envelope = match msg {
            WaitResult::Message(envelope) =>  {
                envelope
            }
            WaitResult::Lagged(i) => {
                warn!("Lagged {} messages", i);
                continue;
            }
        };

        let id = match envelope.id() {
            Id::Extended(e) => {
                e.as_raw()
            }
            Id::Standard(s) => {
                warn!("Received standard (non-extended) CAN ID: {}", s.as_raw());
                continue;
            }
        };

        let fsm_event = match id {
            0x00000001 => fsm::commons::Event::HighVoltageOnCanRelay,
            _ => fsm::commons::Event::NoEvent,
        };

        if fsm_event != fsm::commons::Event::NoEvent {
            event_channel.add_event(&fsm_event).await;
        }
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

    let mut configurator = can::CanConfigurator::new(p.FDCAN1, p.PD0, p.PD1, Irqs);

    // hprintln!("{:?}", configurator.config().nbtr);
    // hprintln!("{:?}", configurator.config().dbtr);
    //NominalBitTiming { prescaler: 12, seg1: 8, seg2: 1, sync_jump_width: 1 }
    // DataBitTiming { transceiver_delay_compensation: true, prescaler: 2, seg1: 8,
    // seg2: 1, sync_jump_width: 1 }

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

    // hprintln!("Generated config: {:?}", configurator.config());

    let mut can = configurator.into_normal_mode();

    // TODO: figure out if we want buffered can
    // let mut can = can.buffered_fd(unsafe{&mut write_buffer}, unsafe{&mut
    // read_buffer});

    // let mut can = can.into_internal_loopback_mode();
    // let mut can = can.into_normal_mode();

    let can = CanInterface::new(can, spawner);

    info!("CAN Configured");

    spawner
        .spawn(run_fsm(spawner, event_channel, emergency_channel, fsm_state))
        .unwrap();

    info!("FSM started!");

    // Generate random seed.
    let mut rng = Rng::new(p.RNG, Irqs);
    let mut seed = [0; 8];
    rng.fill_bytes(&mut seed);
    let seed = u64::from_le_bytes(seed);

    let mac_addr = [0x00, 0x07, 0xE9, 0x42, 0xAC, 0x28];

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

    let gs_master = GsMaster::new(EthernetGsCommsLayerInitializer::new(device, config), spawner).await;

    let gsrx = gs_master.subscribe();
    let gstx = gs_master.transmitter();


    let prio = PriorityEventPubSub::new(
        event_channel.publisher().unwrap(),
        event_channel.subscriber().unwrap(),
        emergency_channel.publisher().unwrap(),
        emergency_channel.subscriber().unwrap(),
    );

    unwrap!(spawner.spawn(forward_gs_to_fsm(gsrx, prio)));

    let prio = PriorityEventPubSub::new(
        event_channel.publisher().unwrap(),
        event_channel.subscriber().unwrap(),
        emergency_channel.publisher().unwrap(),
        emergency_channel.subscriber().unwrap(),
    );
    unwrap!(spawner.spawn(forward_fsm_relay_events_to_can(can.new_sender(), prio)));

    let prio = PriorityEventPubSub::new(
        event_channel.publisher().unwrap(),
        event_channel.subscriber().unwrap(),
        emergency_channel.publisher().unwrap(),
        emergency_channel.subscriber().unwrap(),
    );
    unwrap!(spawner.spawn(forward_can_messages_to_fsm(can.new_subscriber(), prio)));

    loop {
        let i1 = Instant::now();
        for _ in 0..1024 {
            gstx.send(PodToGsMessage {}).await;
        }
        gstx.send(PodToGsMessage {}).await;
        let i2 = Instant::now();

        let diff = i2 - i1;
        info!("wrote 8192 bytes in {}us", diff.as_micros());
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
