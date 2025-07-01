//! This module contains the struct used for the Main FSM.

use core::fmt::Debug;
use core::fmt::Formatter;

use embassy_stm32::gpio::Output;
use embassy_time::Timer;
use lib::Event;
use lib::EventReceiver;
use lib::EventSender;
use lib::States;

use crate::entry_methods::enter_fault;
use crate::CheckedSystem;
use crate::CheckedSystems;

/// The struct for the `MainFSM`
pub struct FSM {
    /// The state in which the pod is in
    state: States,
    /// Object used for receive access to the event channel
    event_receiver: EventReceiver,
    /// Object used to send message over the second CAN bus
    event_sender2: EventSender,
    /// Object used to send messages to the ground station
    event_sender_gs: EventSender,
    /// The systems that should be checked in the `SystemCheck` state
    systems: CheckedSystems,
    /// The pin on the main PCB used for rearming the sdc
    rearm_sdc_pin: Output<'static>,
    /// The pin on the main PCB used for controlling the sdc
    sdc_pin: Output<'static>,
}

impl Debug for FSM {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "FSM {{ state: {:?}}}", self.state)
    }
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
        rearm_sdc_pin: Output<'static>,
        sdc_pin: Output<'static>,
    ) -> Self {
        Self {
            state: States::Boot,
            event_receiver,
            event_sender2,
            event_sender_gs,
            systems: CheckedSystems {
                powertrain: false,
                levitation: false,
                propulsion: false,
            },
            rearm_sdc_pin,
            sdc_pin,
        }
    }

    /// Executes an infinite loop which checks for
    /// events on the `PriorityChannel` and handles them using the
    /// `handle_events` method. It stops the loop if `handle_events` returns
    /// false. This case should only happen if the FSM receives
    /// the `ShutDown` event.
    pub async fn run(&mut self) {
        let mut counter = 0;
        loop {
            let event = self.event_receiver.receive().await;

            if self.state != States::Braking && event != Event::Stopped
                || self.state == States::Braking
            {
                defmt::info!(
                    "FSM {{ state: {} }}: Received event: {:?}",
                    self.state,
                    event
                );
            }

            if !self.handle_events(event).await {
                defmt::info!("{}: Stopping", core::any::type_name::<Self>());
                break;
            }

            if counter <= 100 {
                self.event_sender_gs
                    .send(Event::FSMTransition(self.state.to_index()))
                    .await;
                counter = 0;
            }

            Timer::after_millis(1).await;
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

                // Trigger emergency using the sdc
                self.sdc_pin.set_low();

                // If going in emergency state, send messages over CAN and to the ground station
                self.event_sender2
                    .send(Event::Emergency { emergency_type })
                    .await;
                self.event_sender_gs
                    .send(Event::Emergency { emergency_type })
                    .await;
            }
            (_, Event::Fault) => self.transition(States::Fault).await,

            (_, Event::ResetFSM) => {
                self.transition(States::Boot).await;
                cortex_m::peripheral::SCB::sys_reset();
            }

            (States::Fault, Event::FaultFixed) => self.transition(States::SystemCheck).await,
            (States::Boot, Event::ConnectToGS) => self.transition(States::ConnectedToGS).await,
            (States::ConnectedToGS, Event::StartSystemCheck) => {
                self.transition(States::SystemCheck).await
            }

            (States::SystemCheck, Event::PropSystemCheckSuccess) => {
                self.add_system_check(CheckedSystem::Propulsion).await
            }
            (States::SystemCheck, Event::PowertrainSystemCheckSuccess) => {
                self.add_system_check(CheckedSystem::Powertrain).await
            }
            (States::SystemCheck, Event::LeviSystemCheckSuccess) => {
                self.add_system_check(CheckedSystem::Levitation).await
            }

            (States::Idle, Event::StartPreCharge) => self.transition(States::PreCharge).await,
            (States::PreCharge, Event::HVOnAck) => self.transition(States::Active).await,
            (States::Active, Event::Charge) => self.transition(States::Charging).await,
            (States::Charging, Event::StopCharge) => self.transition(States::Active).await,
            (States::Active, Event::EnterDemo) => {
                self.sdc_pin.set_high();
                Timer::after_millis(100).await;
                self.transition(States::Demo).await;
                self.rearm_sdc_pin.set_high();
                Timer::after_millis(100).await;
                self.rearm_sdc_pin.set_low();
            }
            (States::Demo, Event::Discharge) => self.transition(States::Discharge).await,
            (States::Demo, Event::Levitate) => self.transition(States::Levitating).await,
            (States::Levitating, Event::StopLevitating) => self.transition(States::Demo).await,
            (States::Levitating, Event::Accelerate) => self.transition(States::Accelerating).await,
            (States::Accelerating, Event::Brake) => self.transition(States::Braking).await,
            (States::Braking, Event::Stopped) => self.transition(States::Levitating).await,
            (States::Discharge, Event::EnterIdle) => self.transition(States::Idle).await,
            (States::Discharge, Event::ShutDown) => return false,

            // Send a message to the GS with the state in which it failed to transition in case it
            // receives a wrong event. Doesn't apply for `Event::Stopped` since that is sent
            // whenever we get velocity 0, which should be the case for every state except
            // `States::Accelerating` and `States::Braking`
            (_, event) => {
                if let Some(failed_state_transition) = match event {
                    Event::Emergency { emergency_type: _ } | Event::Fault => Some(States::Fault),
                    Event::ResetFSM => Some(States::Boot),
                    Event::FaultFixed => Some(States::SystemCheck),
                    Event::StartSystemCheck => Some(States::SystemCheck),
                    Event::StartPreCharge => Some(States::PreCharge),
                    Event::HVOnAck => Some(States::Active),
                    Event::Charge => Some(States::Charging),
                    Event::StopCharge => Some(States::Active),
                    Event::EnterDemo => Some(States::Demo),
                    Event::Discharge => Some(States::Discharge),
                    Event::Levitate => Some(States::Levitating),
                    Event::StopLevitating => Some(States::Demo),
                    Event::Accelerate => Some(States::Accelerating),
                    Event::Brake => Some(States::Braking),
                    Event::EnterIdle => Some(States::Idle),
                    Event::ShutDown => Some(States::Boot),
                    _ => None,
                } {
                    self.event_sender_gs
                        .send(Event::TransitionFail(failed_state_transition.to_index()))
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

    /// Marks one of the subsystems as checked while in the `SystemCheck` state.
    /// If all of them are checked, marks them as false for the next system
    /// check and transitions to the `Idle` state.
    async fn add_system_check(&mut self, system: CheckedSystem) {
        match system {
            CheckedSystem::Levitation => {
                self.systems.levitation = true;
                self.event_sender_gs
                    .send(Event::LeviSystemCheckSuccess)
                    .await;
            }
            CheckedSystem::Powertrain => {
                self.systems.powertrain = true;
                self.event_sender_gs
                    .send(Event::PowertrainSystemCheckSuccess)
                    .await;
            }
            CheckedSystem::Propulsion => {
                self.systems.propulsion = true;
                self.event_sender_gs
                    .send(Event::PropSystemCheckSuccess)
                    .await;
            }
        }

        if self.systems.propulsion && self.systems.levitation && self.systems.powertrain {
            self.transition(States::Idle).await;
            self.systems.levitation = false;
            self.systems.propulsion = false;
            self.systems.powertrain = false;
        }
    }

    /// Transitions the FSM to a new state while executing the
    /// former state's exit method and the new state's entry method.
    async fn transition(&mut self, new_state: States) {
        self.call_exit_method(self.state).await;

        self.state = new_state;

        self.event_sender2
            .send(Event::FSMTransition(new_state.to_index()))
            .await;

        self.event_sender_gs
            .send(Event::FSMTransition(new_state.to_index()))
            .await;

        self.call_entry_method(self.state).await;
        defmt::info!("Transitioned to state {}", self.state);
    }

    /// Matches a state with its entry method and executes it. Should be called
    /// whenever a transition happens.
    async fn call_entry_method(&mut self, state: States) {
        match state {
            States::Fault => enter_fault().await,
            States::Boot => {
                // Reset PCB here
                // SEND extra "restarting..." msg to gs
                self.systems.propulsion = false;
                self.systems.levitation = false;
                self.systems.powertrain = false;
            }
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
