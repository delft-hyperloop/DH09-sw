//! This module contains enums and structs shared among the FSMs, as well as
//! their implementations.

/// Enum representing different types of events that the FSMs should handle.
///
/// Since events are sent over a priority channel,
/// they are prioritised by the order they're defined in here,
/// with events defined earlier being higher priority.
#[derive(Clone, PartialEq, Eq, Debug, Copy, defmt::Format, PartialOrd, Ord)]
#[repr(u8)]
pub enum Event {
    /// Emergency event that must trigger the emergency braking system
    Emergency {
        /// The type of emergency
        emergency_type: EmergencyType,
    },
    /// Triggered whenever the PTC goes into failure
    PTCFailure,
    /// Propulsion motor 2 failed the system check
    Prop2SystemCheckFailure,
    /// Propulsion motor 1 failed the system check
    Prop1SystemCheckFailure,
    /// Levi failed the system check
    LeviSystemCheckFailure,
    /// Event sent by the FSM whenever a transition fails
    /// - `u8`: The state in which the FSM didn't transition.
    TransitionFail(u8),
    /// Connection to the Ground Station has been established
    ConnectToGS,
    /// Pressure readings indicate that the EBS should be deployed
    EbsPressureDeployed,
    /// Pressure readings indicate that the EBS should be retracted  
    EbsPressureRetracted,
    /// Start system check
    StartSystemCheck,
    /// Enters `Idle` state from `Discharge` state
    EnterIdle,
    /// Starts the pre-charging process
    StartPreCharge,
    /// Will turn on high voltage while SDC is closed and brakes are deployed
    HVOnAck,
    /// Ack used after discharge to go back to idle
    PTCIdleAck,
    /// Enters the demo state armed brakes, SDC still closed
    EnterDemo,
    /// Command from the ground station to start levitating. Will wait for
    /// acknowledgement.
    Levitate,
    /// Command from the ground station to stop levitating. Will wait for
    /// acknowledgement.
    StopLevitating,
    /// Starts accelerating
    Accelerate,
    /// Brakes with the motor
    Brake,
    /// Used for transitioning from braking to levitating when the speed of the
    /// pod is 0
    Stopped,
    /// Starts discharging
    Discharge,
    /// Shuts down the pod (?)
    ShutDown,
    /// Pod should start charging
    Charge,
    /// Pod should stop charging
    StopCharge,
    /// Resets the FSM to the `Boot` state
    ResetFSM,
    /// Used to transition from `Fault` to `SystemCheck` when the fault is fixed
    /// and no reboot is required
    FaultFixed,
    /// Event sent when transitioning. Used to send the `FSMUpdate` CAN message.
    /// - `u8`: State in which the FSM transitioned
    FSMTransition(u8),
    /// Acknowledgement received from levi that we are levitating
    LeviOnAck,
    /// Acknowledgement that levi is off (not levitating)
    LeviOffAck,
    /// Acknowledgement received from the first propulsion motor that their FSM
    /// also transitioned to new state
    PropulsionAck1,
    /// Acknowledgement received from the second propulsion motor that their FSM
    /// also transitioned to new state
    PropulsionAck2,
    /// Acknowledgement received from powertrain that their FSM also
    /// transitioned to new state
    PowertrainAck,
    /// Acknowledgement for levi fault clear
    ClearFaultAckLevi,
    /// Acknowledgement that levi passed the system check
    LeviSystemCheckSuccess,
    /// Acknowledgement that propulsion motor 1 passed the system check
    Prop1SystemCheckSuccess,
    /// Acknowledgement that propulsion motor 2 passed the system check
    Prop2SystemCheckSuccess,
    /// Override event for rearming the sdc (only used for testing)
    OverrideRearmSdc,
    /// <3
    Heartbeat,
    /// No event happened
    NoEvent,

    /// Used as upper bound when transmuting
    #[doc(hidden)]
    __GUARD,
}

/// Enum for different types of emergencies
#[derive(Clone, PartialEq, Eq, Debug, Copy, defmt::Format, PartialOrd, Ord)]
#[repr(u8)]
pub enum EmergencyType {
    /// General emergency
    GeneralEmergency = 0,
    /// Emergency triggered by propulsion
    EmergencyPropulsion,
    /// Emergency triggered by levitation
    EmergencyLevitation,
    /// Emergency triggered by the powertrain controller
    EmergencyPTC,
    /// Emergency triggered by the BMS
    EmergencyBMS,
    /// Emergency triggered by SenseCon
    EmergencySenseCon,
    /// Emergency triggered by the Sensor Hub
    EmergencySensorHub,
    /// Emergency triggered when we lose connection to the main PCB
    DisconnectionEmergency,
    /// Emergency triggered when the EBS is in the wrong state, measured with
    /// low pressure.
    EmergencyWrongEbsState,
    /// Emergency triggered if one of the critical datapoints has been stale for
    /// more than one second
    StaleCriticalDataEmergency,
}

impl Event {
    const fn guard_event_tag() -> u8 {
        unsafe { core::mem::transmute::<Event, [u8; 2]>(Event::__GUARD)[0] }
    }

    /// Reads from the provided buffer and returns an `Option` containing the
    /// event or `None` if the element transmutes to a value higher than the
    /// `Event` enum limit.
    ///
    /// # Parameters:
    /// - `buf`: the buffer to read from
    ///
    /// # Returns:
    /// - `Option` containing a variant of the `Event` enum
    pub fn read_from_buf(buf: [u8; 2]) -> Option<Self> {
        if buf[0] >= Self::guard_event_tag() {
            return None;
        }

        let event = unsafe { core::mem::transmute_copy::<[u8; 2], Event>(&buf) };

        Some(event)
    }
}
