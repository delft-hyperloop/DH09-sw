//! This module contains the struct used for the Main FSM.

use core::cmp::PartialEq;
use core::future::Future;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use defmt::Format;
use MainStates::*;

use crate::commons::data::Event;
use crate::commons::data::PriorityEventPubSub;
use crate::commons::traits::Runner;
use crate::commons::traits::Transition;
use crate::commons::EmergencyChannel;
use crate::commons::EventChannel;
use crate::define_fsm;
use crate::emergency_fsm::EmergencyFSM;
use crate::high_voltage_fsm::HighVoltageFSM;
use crate::impl_transition;
use crate::levitation_fsm::LevitationFSM;
use crate::operating_fsm::OperatingFSM;
use crate::propulsion_fsm::PropulsionFSM;

/// Enum representing the different states that the `MainFSM` will be in
#[derive(Eq, PartialEq, Debug, Clone, Copy, Format)]
#[allow(dead_code)]
pub enum MainStates {
    /// State for checking the system
    SystemCheck,
    /// Idle state
    Idle,
    /// State for charging the pod
    Charging,
    /// Active state
    Active,
    /// Probably shouldn't be here at all
    FlashingCode,
    /// Operating state where the sub-FSMs are running
    Operating,
}

/// The struct for the `MainFSM`
///
/// # Fields:
/// - `state`: The state in which the pod is in
/// - `priority_event_pub_sub`: struct used for publishing and listening to
///   events
#[derive(Debug)]
pub struct MainFSM {
    state: &'static Mutex<NoopRawMutex, MainStates>,
    priority_event_pub_sub: PriorityEventPubSub,
}

/// Embassy signal used for running the sub-FSMs.
///
/// The `MainFSM` will send a signal to this upon entering the `Operating`
/// state.
static RUN_HV_FSM: Signal<CriticalSectionRawMutex, bool> = Signal::new();
static RUN_EMERGENCY_FSM: Signal<CriticalSectionRawMutex, bool> = Signal::new();
static RUN_OPERATING_FSM: Signal<CriticalSectionRawMutex, bool> = Signal::new();
static RUN_PROPULSION_FSM: Signal<CriticalSectionRawMutex, bool> = Signal::new();
static RUN_LEVITATION_FSM: Signal<CriticalSectionRawMutex, bool> = Signal::new();

/// Atomic bools used to expose the states of the sub-FSMs to each other. They
/// indicate whether the system is active or not.
pub(crate) static HIGH_VOLTAGE_STATE: AtomicBool = AtomicBool::new(false);
pub(crate) static LEVITATION_STATE: AtomicBool = AtomicBool::new(false);
pub(crate) static PROPULSION_STATE: AtomicBool = AtomicBool::new(false);
pub(crate) static EMERGENCY_STATE: AtomicBool = AtomicBool::new(false);

impl MainFSM {
    /// Constructor for the `MainFSM` struct. Defines the sub-FSMs and spawns
    /// embassy tasks for each one of them.
    ///
    /// # Parameters:
    /// - `spawner`: The embassy spawner used to spawn the sub-FSM tasks
    /// - `state`: Static reference to a mutex containing the state of the `MainFSM`
    /// - `event_channel`: Static reference to the channel used for broadcasting
    ///   normal events
    /// - `emergency_channel`: Static reference to the channel used for
    ///   broadcasting emergency events
    ///
    /// # Returns:
    /// - A future for an instance of the `MainFSM` struct
    pub async fn new(
        spawner: Spawner,
        // peripherals: // TODO: add peripherals
        state: &'static Mutex<NoopRawMutex, MainStates>,
        event_channel: &'static EventChannel,
        emergency_channel: &'static EmergencyChannel,
    ) -> Self {
        let high_voltage_fsm = define_fsm!(HighVoltageFSM, event_channel, emergency_channel);
        let emergency_fsm = define_fsm!(EmergencyFSM, event_channel, emergency_channel);
        let operating_fsm = define_fsm!(OperatingFSM, event_channel, emergency_channel);
        let propulsion_fsm = define_fsm!(PropulsionFSM, event_channel, emergency_channel);
        let levitation_fsm = define_fsm!(LevitationFSM, event_channel, emergency_channel);

        spawner
            .spawn(run_high_voltage_fsm(high_voltage_fsm))
            .unwrap();
        spawner.spawn(run_emergency_fsm(emergency_fsm)).unwrap();
        spawner.spawn(run_operating_fsm(operating_fsm)).unwrap();
        spawner.spawn(run_propulsion_fsm(propulsion_fsm)).unwrap();
        spawner.spawn(run_levitation_fsm(levitation_fsm)).unwrap();

        *state.lock().await = MainStates::SystemCheck;

        Self {
            state,
            priority_event_pub_sub: PriorityEventPubSub::new(
                event_channel.publisher().unwrap(),
                event_channel.subscriber().unwrap(),
                emergency_channel.publisher().unwrap(),
                emergency_channel.subscriber().unwrap(),
            ),
        }
    }

    /// Handles the events published to the event channel or the emergency
    /// channel
    ///
    /// This method transitions the `MainFSM` from one state to another
    /// depending on which state it currently is in and what event it
    /// received. If it receives an event that it wasn't expecting in the
    /// current state or if it's meant for one of the sub-FSMs, it ignores
    /// it.
    ///
    /// # Parameters:
    /// - `event`: Event that can cause a transition in the FSM.
    ///
    /// # Returns:
    /// - `false`: If the FSM receives a `Quit` event
    /// - `true`: Otherwise
    async fn handle(&mut self, event: Event) -> bool {
        let state = self.state.lock().await.clone();
        match (state, event) {
            (Operating, Event::Emergency) => {
                return false;
            } // Nothing else needs to be done here, it will be handled by the sub-FSMs
            (_, Event::Emergency) => {
                if !HIGH_VOLTAGE_STATE.load(Ordering::Relaxed) {
                    // TODO: Send CAN command to turn off high voltage
                }
                return false;
            }
            (Operating, Event::StopFSM) => {
                shut_down(&self.priority_event_pub_sub).await;
                self.priority_event_pub_sub
                    .event_channel_publisher
                    .publish(Event::StopSubFSMs)
                    .await;
                return false;
            }
            (_, Event::StopFSM) => return false,
            (SystemCheck, Event::SystemCheckSuccess) => self.transition(Idle, None).await,
            (Idle, Event::Activate) => self.transition(Active, None).await,
            (Idle, Event::Charge) => self.transition(Charging, None).await,
            (Charging, Event::StopCharge) => self.transition(Idle, None).await,
            (Active, Event::Operate) => self.transition(Operating, None).await,
            _ => {}
        }
        true
    }

    /// Returns the current state of the pod.
    pub async fn get_state(&self) -> MainStates {
        let current_state = self.state.lock().await;
        current_state.clone()
    }
}

/// Shuts down all systems in the following order: Propulsion, Levitation, High
/// Voltage.
///
/// Sends CAN commands to shut down the systems inside the pod one by one.
pub async fn shut_down(pub_sub_channels: &PriorityEventPubSub) {
    pub_sub_channels
        .event_channel_publisher
        .publish(Event::PropulsionOff)
        .await;
    loop {
        if !PROPULSION_STATE.load(Ordering::Relaxed) {
            pub_sub_channels
                .event_channel_publisher
                .publish(Event::LevitationOff)
                .await;
            break;
        }
    }
    loop {
        if !LEVITATION_STATE.load(Ordering::Relaxed) {
            pub_sub_channels
                .event_channel_publisher
                .publish(Event::HighVoltageOff)
                .await;
            break;
        }
    }
    loop {
        if !HIGH_VOLTAGE_STATE.load(Ordering::Relaxed) {
            break;
        }
    }
}

/// Runs the propulsion FSM in an embassy task after it receives a signal from
/// the main FSM.
#[embassy_executor::task]
pub async fn run_propulsion_fsm(mut propulsion_fsm: PropulsionFSM) {
    RUN_PROPULSION_FSM.wait().await;
    propulsion_fsm.run().await;
}

/// Runs the levitation FSM in an embassy task after it receives a signal from
/// the main FSM.
#[embassy_executor::task]
pub async fn run_levitation_fsm(mut levitation_fsm: LevitationFSM) {
    RUN_LEVITATION_FSM.wait().await;
    levitation_fsm.run().await;
}

/// Runs the propulsion FSM in an embassy task after it receives a signal from
/// the main FSM.
#[embassy_executor::task]
pub async fn run_high_voltage_fsm(mut high_voltage_fsm: HighVoltageFSM) {
    RUN_HV_FSM.wait().await;
    high_voltage_fsm.run().await;
}

/// Runs the operating FSM in an embassy task after it receives a signal from
/// the main FSM.
#[embassy_executor::task]
pub async fn run_operating_fsm(mut operating_fsm: OperatingFSM) {
    RUN_OPERATING_FSM.wait().await;
    operating_fsm.run().await;
}

/// Runs the emergency FSM in an embassy task after it receives a signal from
/// the main FSM.
#[embassy_executor::task]
pub async fn run_emergency_fsm(mut emergency_fsm: EmergencyFSM) {
    RUN_EMERGENCY_FSM.wait().await;
    emergency_fsm.run().await;
}

// impl_runner_get_sub_channel!(MainFSM);
// impl_transition!(MainFSM, MainStates);

impl Runner for MainFSM {
    fn get_pub_sub_channel(&mut self) -> &mut PriorityEventPubSub {
        &mut self.priority_event_pub_sub
    }

    fn handle_events(&mut self, event: Event) -> impl Future<Output=bool> {
        self.handle(event)
    }
}

impl_transition!(MainFSM, MainStates,
    GetState: get_state,
    SetState: set_state,

    OnEntry:
    // SystemCheck => noop,
    // Idle => noop,
    // Charging => noop,
    Active => enter_active,
    // FlashingCode => noop,
    Operating => enter_operating,
);

async fn get_state(main_fsm: &MainFSM) -> MainStates {
    main_fsm.get_state().await
}

async fn set_state(main_fsm: &mut MainFSM, state: MainStates) {
    *main_fsm.state.lock().await = state;
}

/// Signals the tasks tied to each sub-FSM that they should start running.
async fn enter_operating(main_fsm: &mut MainFSM) {
    RUN_EMERGENCY_FSM.signal(true);
    RUN_OPERATING_FSM.signal(true);
    RUN_PROPULSION_FSM.signal(true);
    RUN_LEVITATION_FSM.signal(true);
    RUN_HV_FSM.signal(true);
}

async fn enter_active(main_fsm: &mut MainFSM) {
    // TODO: Send CAN command to turn on high voltage
    HIGH_VOLTAGE_STATE.store(true, Ordering::Relaxed);
    // TODO: Close SDC while keeping brakes engaged
}