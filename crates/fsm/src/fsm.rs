//! This module contains the struct used for the Main FSM.

use core::cmp::PartialEq;
use defmt::Format;
use States::*;

use crate::commons::data::Event;
use crate::commons::traits::Transition;
use crate::commons::types::{EventReceiver, EventSender};

/// Enum representing the different states that the `MainFSM` will be in
#[derive(Eq, PartialEq, Debug, Clone, Copy, Format)]
#[allow(dead_code)]
pub enum States {
    /// Initial state of the FSM
    Boot,
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
    /// Fault/Emergency state - can be reached from any state and must cause emergency brake
    Fault,
}

/// The struct for the `MainFSM`
///
/// # Fields:
/// - `state`: The state in which the pod is in
/// - `event_receiver`: Object used for receive access to the event channel
/// - `event_sender`: Object used for send access to the event channel
#[derive(Debug)]
pub struct FSM {
    state: States,
    event_receiver: EventReceiver,
    event_sender: EventSender,
}


impl FSM {
    /// Constructor for the `FSM` struct. Initializes the FSM in the `Boot` state.
    ///
    /// # Parameters:
    /// - `state`: Static reference to a mutex containing the state of the `FSM`
    /// - `event_channel`: Static reference to the channel used for broadcasting
    ///   normal events
    /// - `emergency_channel`: Static reference to the channel used for
    ///   broadcasting emergency events
    ///
    /// # Returns:
    /// - A future for an instance of the `MainFSM` struct
    pub async fn new(
        // peripherals: // TODO: add peripherals
        event_sender: EventSender,
        event_receiver: EventReceiver,
    ) -> Self {
        Self {
            state: Boot,
            event_sender,
            event_receiver,
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
    async fn handle(&mut self, event: Event) -> bool {
        match (self.state, event) {
            (_, Event::Emergency) => self.transition(States::Fault).await,
            (_, Event::EmergencyLevitation) => self.transition(States::Fault).await,
            (_, Event::EmergencyPropulsion) => self.transition(States::Fault).await,
            (_, Event::EmergencyPTC) => self.transition(States::Fault).await,
            (_, Event::Fault) => self.transition(States::Fault).await,

            (States::Boot, Event::ConnectToGS) => self.transition(States::ConnectedToGS).await,
            (States::ConnectedToGS, Event::StartSystemCheck) => self.transition(States::SystemCheck).await,
            (States::SystemCheck, Event::SystemCheckSuccess) => self.transition(States::Idle).await,
            (States::Idle, Event::StartPreCharge) => self.transition(States::PreCharge).await,
            (States::PreCharge, Event::Activate) => self.transition(States::Active).await,
            (States::Active, Event::Charge) => self.transition(States::Charging).await,
            (States::Charging, Event::StopCharge) => self.transition(States::Active).await,
            (States::Active, Event::EnterDemo) => self.transition(States::Demo).await,
            (States::Demo, Event::Discharge) => self.transition(States::Discharge).await,
            (States::Demo, Event::Levitate) => self.transition(States::Levitating).await,
            (States::Levitating, Event::StopLevitating) => self.transition(States::Demo).await,
            (States::Levitating, Event::Accelerate {target_speed}) => self.transition(States::Accelerating).await,
            (States::Accelerating, Event::Cruise) => self.transition(States::Cruising).await,
            (States::Accelerating, Event::Brake) => self.transition(States::Braking).await,
            (States::Cruising, Event::Brake) => self.transition(States::Braking).await,
            (States::Braking, Event::Stopped) => self.transition(States::Levitating).await,
            (States::Discharge, Event::EnterIdle) => self.transition(States::Idle).await,
            (States::Discharge, Event::ShutDown) => return false,
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
    pub async fn transition(&mut self, new_state: States) {
        // TODO: Exit method
        self.state = new_state;
        // TODO: Entry method
    }
}