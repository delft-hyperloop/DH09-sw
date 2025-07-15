//! Methods to match between CAN datapoints, FSM events and ground station
//! commands.

use defmt::error;
use lib::config::Command;
use lib::config::Datatype;
use lib::EmergencyType;
use lib::Event;

/// Matches a CAN ID from the received data to an event for the FSM. This is
/// different to the method in `config.rs` because it returns an event based on
/// the payload or an emergency which also requires the type of emergency.
pub fn match_can_id_to_event(id: u32, payload: &[u8]) -> Event {
    match id {
        826 if i32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]) <= 20400 => {
            Event::LocalizationLimitReached
        }

        // Pressure brakes
        // 831 => {
        //     let pressure_low = u16::from_be_bytes([payload[0], payload[1]]);
        //     if pressure_low < 1500 {
        //         Event::EbsPressureDeployed
        //     } else {
        //         Event::EbsPressureRetracted
        //     }
        // }

        // If it gets a ptc logs message from the powertrain controller with state HV
        // on, send ack to fsm
        1251 => match payload[0] {
            0 => Event::PTCIdleAck,
            2 => Event::HVOnAck,
            3 => Event::PTCFailure,
            _ => Event::NoEvent,
        },

        // Check if the velocity is 0, which means that the pod is not moving (used for
        // transitioning from the braking state to the levitating state)
        826 if i16::from_be_bytes([payload[4], payload[5]]) == 0 => Event::Stopped,

        905 if payload[0] == 3 => Event::Emergency {
            emergency_type: EmergencyType::EmergencyLevitation,
        },

        // Levi FSM update ack
        905 if payload[0] == 2 => Event::LeviOnAck,
        905 if payload[0] == 1 => Event::LeviOffAck,

        // Response from levi to the system check
        906 => {
            if payload[0] == 1 {
                Event::LeviSystemCheckSuccess
            } else {
                error!("Different system check payload for levi: {}", payload[0]);
                Event::LeviSystemCheckSuccess // TODO: change this back to failure
            }
        }

        // Response from propulsion motor left (2) to the system check
        876 => {
            if payload[0] == 1 {
                Event::Prop1SystemCheckSuccess
            } else {
                error!(
                    "Different system check payload for left motor: {}",
                    payload[0]
                );
                Event::Prop1SystemCheckFailure
            }
        }
        1286 if payload[0] != 255 => Event::Prop2SystemCheckFailure,

        // Response from propulsion motor right (1) to the system check
        877 => {
            if payload[0] == 1 {
                Event::Prop2SystemCheckSuccess
            } else {
                error!(
                    "Different system check payload for right motor: {}",
                    payload[0]
                );
                Event::Prop2SystemCheckFailure
            }
        }
        1285 if payload[0] != 255 => Event::Prop1SystemCheckFailure,

        // Levi emergency
        101 => Event::Emergency {
            emergency_type: EmergencyType::EmergencyLevitation,
        },

        // Powertrain emergency
        51 => {
            if payload[1] != 0 || payload[2] != 0 {
                Event::Emergency {
                    emergency_type: EmergencyType::EmergencyBMS,
                }
            } else {
                Event::Emergency {
                    emergency_type: EmergencyType::EmergencyPTC,
                }
            }
        }

        // Sensor Hub emergency
        26 => Event::Emergency {
            emergency_type: EmergencyType::EmergencySensorHub,
        },

        _ => Event::NoEvent,
    }
}

/// Matches an event from the FSM to a command and payload, packed into a CAN
/// envelope
pub fn match_event_to_can_envelope(event: Event) -> Option<lib::can::can2::CanEnvelope> {
    match event {
        fsm::Event::FSMTransition(state_number) => Some(lib::can::can2::CanEnvelope::new_with_id(
            Command::FSMUpdate(0).to_id(),
            &[state_number],
        )),
        fsm::Event::Discharge => Some(lib::can::can2::CanEnvelope::new_with_id(
            Command::StopHV(0).to_id(),
            &[0],
        )),
        _ => None,
    }
}

/// Matches an event from the FSM to a datapoint and payload packed into a
/// PodToGsMessage
pub fn match_event_to_datapoint(event: Event) -> Option<(Datatype, u64)> {
    match event {
        Event::Emergency { emergency_type } => {
            Some((Datatype::Emergency, (emergency_type as i32 + 1) as u64))
        }
        Event::FSMTransition(transitioned_state) => {
            Some((Datatype::FSMState, transitioned_state as u64))
        }
        Event::TransitionFail(other_state) => {
            Some((Datatype::FSMTransitionFail, other_state as u64))
        }
        Event::FSMHeartbeat(state) => Some((Datatype::FSMState, state as u64)),
        Event::LeviSystemCheckFailure => Some((Datatype::LeviSystemCheckFailure, 0)),
        Event::LeviSystemCheckSuccess => Some((Datatype::LeviSystemCheckSuccess, 0)),
        Event::Prop1SystemCheckFailure => Some((Datatype::Prop1SystemCheckFailure, 0)),
        Event::Prop1SystemCheckSuccess => Some((Datatype::Prop1SystemCheckSuccess, 0)),
        Event::Prop2SystemCheckSuccess => Some((Datatype::Prop2SystemCheckSuccess, 0)),
        Event::Prop2SystemCheckFailure => Some((Datatype::Prop2SystemCheckFailure, 0)),
        Event::ResetFSM => Some((Datatype::ResetFSM, 1)),
        Event::LocalizationLimitReached => Some((Datatype::LocalizationLimitReached, 0)),
        _ => None,
    }
}

/// Matches a `Command` to and FSM `Event`
///
/// -`command`: The command that should be matched to an event
pub fn match_cmd_to_event(command: Command) -> Event {
    match command {
        // General commands
        Command::GeneralEmergency(_) => Event::Emergency {
            emergency_type: lib::EmergencyType::GeneralEmergency,
        },
        Command::EmergencyBrake(_) => Event::Emergency {
            emergency_type: lib::EmergencyType::GeneralEmergency,
        },
        Command::FaultFixed(_) => Event::FaultFixed,
        Command::ReconnectEmergency(_) => Event::Emergency {
            emergency_type: EmergencyType::DisconnectionEmergency,
        },

        // HV commands
        Command::StartHV(_) => Event::StartPreCharge,
        Command::StopHV(_) => Event::Discharge,
        Command::Charge(_) => Event::Charge,
        Command::StopCharge(_) => Event::StopCharge,

        // Levi commands
        Command::LevitationOn(_) => Event::Levitate,
        Command::LevitationOff(_) => Event::StopLevitating,

        // Propulsion commands
        Command::PropulsionOn(_) => Event::Accelerate,
        Command::MotorBrake(_) => Event::Brake,

        // Control commands
        Command::SystemCheck(_) => Event::StartSystemCheck,
        Command::Shutdown(_) => Event::ShutDown,
        Command::RearmSDC(_) => Event::EnterDemo,

        // Reset commands
        Command::SystemReset(_) => Event::ResetFSM,
        Command::ResetSenseCon(_) => Event::ResetFSM,

        Command::ConnectionEstablished(_) => Event::ConnectToGS,

        // These were used strictly for testing purposes. They should not be used during a normal
        // run.
        Command::MockLeviAck(_) => Event::LeviSystemCheckSuccess,
        Command::MockProp1Ack(_) => Event::Prop1SystemCheckSuccess,
        Command::MockProp2Ack(_) => Event::Prop2SystemCheckSuccess,
        Command::MockHVOn(_) => Event::HVOnAck,

        Command::FailLeviSystemCheck(_) => Event::LeviSystemCheckFailure,
        Command::FailProp1SystemCheck(_) => Event::Prop1SystemCheckFailure,
        Command::FailProp2SystemCheck(_) => Event::Prop2SystemCheckFailure,

        Command::OverrideRearmSdc(_) => Event::OverrideRearmSdc,

        Command::FrontendHeartbeat(_) => Event::Heartbeat,

        _ => Event::NoEvent,
    }
}
