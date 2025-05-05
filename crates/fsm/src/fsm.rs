//! This module contains the struct used for the Main FSM.

use core::cmp::PartialEq;

use defmt::Format;
#[cfg(test)]
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
#[cfg(test)]
use embassy_sync::mutex::Mutex;
use States::*;

use crate::entry_methods::enter_fault;
use crate::utils::data::Event;
use crate::utils::types::EventReceiver;
use crate::utils::types::EventSender;

/// Enum representing the different states that the `MainFSM` will be in
#[derive(Eq, PartialEq, Debug, Clone, Copy, Format)]
#[allow(dead_code)]
pub enum States {
    /// Initial state of the FSM
    Boot = 0,
    /// Pretty self-explanatory :)
    ConnectedToGS,
    /// State for checking each subsystem
    SystemCheck,
    /// Idle state, SDC is open and emergency brakes are deployed
    Idle,
    /// Pre-charging the batteries before turning on high voltage
    PreCharge,
    /// High voltage is on, SDC is closed, emergency brakes are still deployed
    Active,
    /// SDC closed, brakes not deployed (SDC is armed)
    Demo,
    /// Pod is levitating
    Levitating,
    /// Pod is accelerating
    Accelerating,
    /// Pod is cruising
    Cruising,
    /// Pod is braking
    Braking,
    /// Discharge state for the high voltage current
    Discharge,
    /// State for charging the pod
    Charging,
    /// Fault/Emergency state - can be reached from any state and must cause
    /// emergency brake
    Fault,
}

/// The struct for the `MainFSM`
///
/// # Fields:
/// - `state`: The state in which the pod is in
/// - `event_receiver`: Object used for receive access to the event channel
/// - `event_sender1`: Object used to send message over the first CAN bus
/// - `event_sender2`: Object used to send message over the second CAN bus
#[derive(Debug, Copy, Clone)]
pub struct FSM {
    #[cfg(test)]
    state_mutex: &'static Mutex<NoopRawMutex, States>,
    state: States,
    event_receiver: EventReceiver,
    event_sender1: EventSender,
    event_sender2: EventSender,
}

impl FSM {
    /// Constructor for the `FSM` struct. Initializes the FSM in the `Boot`
    /// state.
    ///
    /// # Parameters:
    /// - `event_receiver`: Static reference to a receiver object from the
    ///   `PriorityChannel` used to transmit events
    ///
    /// # Returns:
    /// - A future for an instance of the `FSM` struct
    pub async fn new(
        // peripherals: // TODO: add peripherals
        event_receiver: EventReceiver,
        event_sender1: EventSender,
        event_sender2: EventSender,
        #[cfg(test)] state_mutex: &'static Mutex<NoopRawMutex, States>,
    ) -> Self {
        Self {
            state: Boot,
            event_receiver,
            event_sender1,
            event_sender2,
            #[cfg(test)]
            state_mutex,
        }
    }

    /// Executes an infinite loop which checks for
    /// events on the `PriorityChannel` and handles them using the
    /// `handle_events` method. It stops the loop if `handle_events` returns
    /// false. This case should only happen if the FSM receives
    /// the `ShutDown` event.
    pub async fn run(&mut self) {
        loop {
            let event = self.event_receiver.receive().await;

            defmt::info!(
                "{}: Received event: {:?}",
                core::any::type_name::<Self>(),
                event
            );

            if !self.handle_events(event).await {
                defmt::info!("{}: Stopping", core::any::type_name::<Self>());
                break;
            }
        }
    }

    /// Handles the events published to the event channel.
    ///
    /// This method transitions the `FSM` from one state to another
    /// depending on which state it currently is in and what event it
    /// received. If it receives an event that it wasn't expecting in the
    /// current state, it ignores it.
    ///
    /// # Parameters:
    /// - `event`: Event that can cause a transition in the FSM.
    ///
    /// # Returns:
    /// - `false`: If the FSM receives a `Quit` event
    /// - `true`: Otherwise
    async fn handle_events(&mut self, event: Event) -> bool {
        match (self.state, event) {
            (_, Event::Emergency { emergency_type: _ }) => self.transition(States::Fault).await,
            (_, Event::Fault) => self.transition(States::Fault).await,

            (States::Fault, Event::FaultFixed) => self.transition(SystemCheck).await,
            (States::Boot, Event::ConnectToGS) => self.transition(States::ConnectedToGS).await,
            (States::ConnectedToGS, Event::StartSystemCheck) => {
                self.transition(States::SystemCheck).await
            }
            (States::SystemCheck, Event::SystemCheckSuccess) => self.transition(States::Idle).await,
            (States::Idle, Event::StartPreCharge) => self.transition(States::PreCharge).await,
            (States::PreCharge, Event::Activate) => self.transition(States::Active).await,
            (States::Active, Event::Charge) => self.transition(States::Charging).await,
            (States::Charging, Event::StopCharge) => self.transition(States::Active).await,
            (States::Active, Event::EnterDemo) => self.transition(States::Demo).await,
            (States::Demo, Event::Discharge) => self.transition(States::Discharge).await,
            (States::Demo, Event::Levitate) => self.transition(States::Levitating).await,
            (States::Levitating, Event::StopLevitating) => self.transition(States::Demo).await,
            (States::Levitating, Event::Accelerate) => self.transition(States::Accelerating).await,
            (States::Accelerating, Event::Cruise) => self.transition(States::Cruising).await,
            (States::Accelerating, Event::Brake) => self.transition(States::Braking).await,
            (States::Cruising, Event::Brake) => self.transition(States::Braking).await,
            (States::Braking, Event::Stopped) => self.transition(States::Levitating).await,
            (States::Discharge, Event::EnterIdle) => self.transition(States::Idle).await,
            (States::Discharge, Event::ShutDown) => return false,

            #[cfg(test)]
            (_, Event::StopFSM) => return false,

            _ => {}
        }
        true
    }

    /// Returns the current state of the pod.
    pub async fn get_state(&self) -> States {
        self.state
    }

    /// Transitions the FSM to a new state while executing the
    /// former state's exit method and the new state's entry method.
    async fn transition(&mut self, new_state: States) {
        self.call_exit_method(self.state).await;

        self.state = new_state;
        #[cfg(test)]
        {
            **self.state_mutex.lock().await = new_state;
        }
        
        self.event_sender2.send(Event::FSMTransition(new_state as u8)).await;

        self.call_entry_method(self.state).await;
        defmt::info!("Transitioned to state {}", self.state);
    }

    /// Matches a state with its entry method and executes it. Should be called
    /// whenever a transition happens.
    async fn call_entry_method(&self, state: States) {
        match state {
            States::Fault => enter_fault().await,
            _ => {}
        }
    }

    ///Matches a state with its exit method and executes it. Should be called
    /// whenever a transition happens
    async fn call_exit_method(&self, state: States) {
        match state {
            _ => {}
        }
    }
}
