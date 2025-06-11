//! This module contains the struct used for the Main FSM.

use lib::Event;
use lib::EventReceiver;
use lib::EventSender;
use lib::States;

use crate::entry_methods::enter_fault;

/// The struct for the `MainFSM`
#[derive(Debug, Copy, Clone)]
pub struct FSM {
    /// The state in which the pod is in
    state: States,
    /// Object used for receive access to the event channel
    event_receiver: EventReceiver,
    /// Object used to send message over the second CAN bus
    event_sender2: EventSender,
    /// Object used to send messages to the groundstations
    event_sender_gs: EventSender,
}

impl FSM {
    /// Constructor for the `FSM` struct. Initializes the FSM in the `Boot`
    /// state.
    ///
    /// # Parameters:
    /// - `event_receiver`: Static reference to a receiver object from the
    ///   `PriorityChannel` used to transmit events
    /// - `event_sender2`: Static reference to a sender object from the
    ///   `PriorityChannel` used to send events on CAN 2
    /// - `event_sender_gs`: Static reference to a sender object from the
    ///   `PriorityChannel` used to send events to the groundstation
    ///
    /// # Returns:
    /// - A future for an instance of the `FSM` struct
    pub async fn new(
        event_receiver: EventReceiver,
        event_sender2: EventSender,
        event_sender_gs: EventSender,
    ) -> Self {
        Self {
            state: States::Boot,
            event_receiver,
            event_sender2,
            event_sender_gs,
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
    /// current state, it ignores it or sends a `TransitionFail` event to the GS
    /// if it was an event related to a state transition.
    ///
    /// # Parameters:
    /// - `event`: Event that can cause a transition in the FSM.
    ///
    /// # Returns:
    /// - `false`: If the FSM receives a `Quit` event
    /// - `true`: Otherwise
    async fn handle_events(&mut self, event: Event) -> bool {
        match (self.state, event) {
            (_, Event::Emergency { emergency_type }) => {
                self.transition(States::Fault).await;

                // If going in emergency state, send messages over CAN and to the groundstation
                self.event_sender2
                    .send(Event::Emergency { emergency_type })
                    .await;
                self.event_sender_gs
                    .send(Event::Emergency { emergency_type })
                    .await;
            }
            (_, Event::Fault) => self.transition(States::Fault).await,

            (_, Event::ResetFSM) => self.transition(States::Boot).await,

            (States::Fault, Event::FaultFixed) => self.transition(States::SystemCheck).await,
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

            // Send a message to the GS with the state in which it failed to transition in case it
            // receives a wrong event
            (_, event) => {
                if let Some(failed_state_transition) = match event {
                    Event::Emergency { emergency_type: _ } | Event::Fault => Some(States::Fault),
                    Event::ResetFSM => Some(States::Boot),
                    Event::FaultFixed => Some(States::SystemCheck),
                    Event::ConnectToGS => Some(States::ConnectedToGS),
                    Event::StartSystemCheck => Some(States::SystemCheck),
                    Event::SystemCheckSuccess => Some(States::Idle),
                    Event::StartPreCharge => Some(States::PreCharge),
                    Event::Activate => Some(States::Active),
                    Event::Charge => Some(States::Charging),
                    Event::StopCharge => Some(States::Active),
                    Event::EnterDemo => Some(States::Demo),
                    Event::Discharge => Some(States::Discharge),
                    Event::Levitate => Some(States::Levitating),
                    Event::StopLevitating => Some(States::Demo),
                    Event::Accelerate => Some(States::Accelerating),
                    Event::Cruise => Some(States::Cruising),
                    Event::Brake => Some(States::Braking),
                    Event::Stopped => Some(States::Levitating),
                    Event::EnterIdle => Some(States::Idle),
                    Event::ShutDown => Some(States::Boot),
                    _ => None,
                } {
                    self.event_sender_gs
                        .send(Event::TransitionFail(failed_state_transition as u8))
                        .await;
                }
            }
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

        self.event_sender2
            .send(Event::FSMTransition(new_state as u8))
            .await;
        self.event_sender_gs
            .send(Event::FSMTransition(new_state as u8))
            .await;

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
