//! This module contains enums and structs shared among the FSMs, as well as
//! their implementations.

/// Enum representing different types of events that the FSMs should handle.
#[derive(Clone, PartialEq, Eq, Debug, Copy, defmt::Format, PartialOrd, Ord)]
#[repr(u8)]
pub enum Event {
    /// No event happened
    NoEvent,
    /// Connection to the Ground Station has been established
    ConnectToGS,
    /// Start system check
    StartSystemCheck,
    /// The System was checked successfully
    SystemCheckSuccess,
    /// Enters `Idle` state from `Discharge` state
    EnterIdle,
    /// Starts the pre-charging process
    StartPreCharge,
    /// Will turn on high voltage while SDC is closed and brakes are deployed
    Activate,
    /// Enters the demo state armed brakes, SDC still closed
    EnterDemo,
    /// Starts levitating
    Levitate,
    /// Stops levitating
    StopLevitating,
    /// Starts accelerating
    Accelerate,
    /// Stops accelerating
    Cruise,
    /// Deploys brakes
    Brake,
    /// Used for transitioning from braking to levitating when the speed of the
    /// pod is 0
    Stopped,
    /// Starts discharging the high voltage batteries
    Discharge,
    /// Shuts down the pod
    ShutDown,
    /// Pod should start charging
    Charge,
    /// Pod should stop charging
    StopCharge,
    /// Resets the FSM to the `Boot` state
    ResetFSM,
    /// Fault happened
    Fault,
    /// Emergency event that must trigger the emergency braking system
    Emergency {
        /// The type of emergency
        emergency_type: EmergencyType,
    },
    /// Used to transition from `Fault` to `SystemCheck` when the fault is fixed
    /// and no reboot is required
    FaultFixed,
    ///
    HighVoltageOnCanRelay,
    /// Event sent when transitioning. Used to send the `FSMUpdate` CAN message.
    /// - `u8`: State in which the FSM transitioned
    FSMTransition(u8),
    /// Event sent by the FSM whenever a transition fails
    /// - `u8`: The state in which the FSM didn't transition.
    TransitionFail(u8),

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
    /// Emergency triggered by SenseCon
    EmergencySenseCon,
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
