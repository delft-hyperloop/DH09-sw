//! This module contains the struct used for the Main FSM.
#![allow(clippy::match_single_binding)]
use core::fmt::Debug;
use core::fmt::Formatter;

use defmt::info;
use embassy_stm32::gpio::Output;
use embassy_time::Timer;
use lib::EmergencyType;
use lib::Event;
use lib::EventReceiver;
use lib::EventSender;
use lib::States;
use log::error;

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
                levitation: false,
                propulsion1: false,
                propulsion2: false,
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
        let mut counter = 0u64;
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

            // Sends the FSM state to the ground station every 100 iterations
            if counter.rem_euclid(100) == 0 {
                self.event_sender_gs
                    .send(Event::FSMTransition(self.state.to_index()))
                    .await;
            }

            // Checks if the braking line is still high. If not, send an emergency message
            // to the ground station and transition to fault state.
            if self.sdc_pin.is_set_low() && self.state != States::Fault {
                error!("SDC pin is low! Sending emergency to the ground station!");
                self.event_sender_gs
                    .send(Event::Emergency {
                        emergency_type: EmergencyType::GeneralEmergency,
                    })
                    .await;
                self.transition(States::Fault).await;
            }

            Timer::after_micros(100).await;
            counter = counter.wrapping_add(1);
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
            (_, Event::Emergency { emergency_type }) if self.state != States::Fault => {
                // 1. Trigger emergency using the sdc
                self.sdc_pin.set_low();
                error!("Going into Fault state with emergency {:?}", emergency_type);

                // 2. send the emergency message to all other devices on the CAN line
                self.event_sender2
                    .send(Event::Emergency { emergency_type })
                    .await;

                // 3. discharge the batteries
                self.event_sender2.send(Event::Discharge).await;

                // 4. inform the ground station about it
                if emergency_type != EmergencyType::StaleCriticalDataEmergency {
                    self.event_sender_gs
                        .send(Event::Emergency { emergency_type })
                        .await;
                }

                // lastly, transition to fault state.
                self.transition(States::Fault).await;
            }

            (_, Event::ResetFSM) => {
                info!("Reset FSM triggered. Resetting the main PCB...");
                self.event_sender_gs.send(Event::ResetFSM).await;
                self.sdc_pin.set_low();
                Timer::after_millis(5).await;
                cortex_m::peripheral::SCB::sys_reset();
            }

            (States::Fault, Event::FaultFixed) => {
                self.sdc_pin.set_high();
                self.transition(States::SystemCheck).await
            }
            (States::Boot, Event::ConnectToGS) => self.transition(States::ConnectedToGS).await,
            (States::ConnectedToGS, Event::StartSystemCheck) => {
                self.transition(States::SystemCheck).await
            }

            // Add the checked system to the list of checked systems
            (States::SystemCheck, Event::Prop1SystemCheckSuccess) => {
                self.add_system_check(CheckedSystem::Propulsion1).await
            }
            (States::SystemCheck, Event::Prop2SystemCheckSuccess) => {
                self.add_system_check(CheckedSystem::Propulsion2).await
            }
            (States::SystemCheck, Event::LeviSystemCheckSuccess) => {
                self.add_system_check(CheckedSystem::Levitation).await
            }

            // Go into fault state if any of the system checks fail
            (States::SystemCheck, Event::Prop1SystemCheckFailure) => {
                error!("Prop 1 system check failure!");
                self.event_sender_gs
                    .send(Event::Prop1SystemCheckFailure)
                    .await;
                self.transition(States::Fault).await;
            }
            (States::SystemCheck, Event::Prop2SystemCheckFailure) => {
                error!("Prop 2 system check failure!");
                self.event_sender_gs
                    .send(Event::Prop2SystemCheckFailure)
                    .await;
                self.transition(States::Fault).await;
            }
            (States::SystemCheck, Event::LeviSystemCheckFailure) => {
                error!("Levi system check failure!");
                self.transition(States::Fault).await;
                self.event_sender_gs
                    .send(Event::LeviSystemCheckFailure)
                    .await;
            }

            (States::Idle, Event::StartPreCharge) => self.transition(States::PreCharge).await,
            (States::PreCharge, Event::HVOnAck) => self.transition(States::Active).await,
            (States::Active, Event::Charge) => self.transition(States::Charging).await,
            (States::Charging, Event::StopCharge) => self.transition(States::Active).await,
            (_, Event::OverrideRearmSdc) => {
                self.sdc_pin.set_high();
                Timer::after_millis(100).await;
                self.rearm_sdc_pin.set_high();
                Timer::after_millis(100).await;
                self.rearm_sdc_pin.set_low();
            }
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
            (States::Idle, Event::ShutDown) => return false,

            (
                States::PreCharge
                | States::Active
                | States::Demo
                | States::Discharge
                | States::Levitating
                | States::Accelerating
                | States::Braking,
                Event::PTCIdleAck,
            ) => self.transition(States::Idle).await,

            // Send a message to the GS with the state in which it failed to transition in case it
            // receives a wrong event. Doesn't apply for `Event::Stopped` since that is sent
            // whenever we get velocity 0, which should be the case for every state except
            // `States::Accelerating` and `States::Braking`
            (_, event) => {
                if let Some(failed_state_transition) = match event {
                    Event::ResetFSM => Some(States::Boot),
                    Event::StartSystemCheck => Some(States::SystemCheck),
                    Event::StartPreCharge => Some(States::PreCharge),
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

    /// Marks one of the subsystems as checked while in the `SystemCheck` state.
    /// If all of them are checked, marks them as false for the next system
    /// check and transitions to the `Idle` state.
    async fn add_system_check(&mut self, system: CheckedSystem) {
        match system {
            CheckedSystem::Levitation => {
                self.systems.levitation = true;
                info!("Levi system check success!");
                self.event_sender_gs
                    .send(Event::LeviSystemCheckSuccess)
                    .await;
            }
            CheckedSystem::Propulsion1 => {
                self.systems.propulsion1 = true;
                info!("Prop1 system check success!");
                self.event_sender_gs
                    .send(Event::Prop1SystemCheckSuccess)
                    .await;
            }
            CheckedSystem::Propulsion2 => {
                self.systems.propulsion2 = true;
                info!("Prop2 system check success!");
                self.event_sender_gs
                    .send(Event::Prop2SystemCheckSuccess)
                    .await;
            }
        }

        if self.systems.propulsion1 && self.systems.propulsion2 && self.systems.levitation {
            self.transition(States::Idle).await;
            self.systems.levitation = false;
            self.systems.propulsion1 = false;
            self.systems.propulsion2 = false;
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
            States::Fault => self.sdc_pin.set_low(),
            States::Boot => {
                // Reset PCB here
                // SEND extra "restarting..." msg to gs
                self.systems.propulsion1 = false;
                self.systems.propulsion2 = false;
                self.systems.levitation = false;
            }
            _ => {}
        }
    }

    /// Matches a state with its exit method and executes it.
    /// Should be called whenever a transition happens
    async fn call_exit_method(&self, state: States) {
        match state {
            _ => {}
        }
    }
}
