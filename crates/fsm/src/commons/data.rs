//! This module contains enums and structs shared among the FSMs, as well as
//! their implementations.

/// Enum representing different types of events that the FSMs should handle.
#[derive(Clone, PartialEq, Debug, Copy, PartialOrd, defmt::Format)]
#[repr(u8)]
pub enum Event {
    /// No event happened
    NoEvent,
    /// Connection to the Ground Station has been established
    ConnectToGS,
    /// Start system check
    StartSystemCheck,
    /// System was checked successfully
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
    /// Starts accelerating until it reaches the target speed
    Accelerate {
        target_speed: u32,
    },
    /// Stops accelerating
    Cruise,
    /// Deploys brakes
    Brake,
    /// Used for transitioning from braking to levitating when the speed of the pod is 0
    Stopped,
    /// Starts discharging the high voltage batteries
    Discharge,
    /// Shuts down the pod
    ShutDown,
    /// Pod should start charging
    Charge,
    /// Pod should stop charging
    StopCharge,
    /// Stops the FSM
    StopFSM,
    /// General emergency happened
    Emergency,
    /// Fault happened
    Fault,
    /// Emergency triggered by propulsion
    EmergencyPropulsion,
    /// Emergency triggered by levitation
    EmergencyLevitation,
    /// Emergency triggered by the powertrain controller
    EmergencyPTC,

    #[doc(hidden)]
    __GUARD,
}

impl Event {
    const fn guard_event_tag() -> u8 {
        unsafe {core::mem::transmute::<Event, [u8; 2]>(Event::__GUARD)[0]}
    }

    pub fn read_from_buf(buf: [u8; 2]) -> Option<Self> {
        if buf[0] >= Self::guard_event_tag() {
            return None;
        }

        let event = unsafe {core::mem::transmute_copy::<[u8; 2], Event>(&buf)};

        Some(event)
    }
}
