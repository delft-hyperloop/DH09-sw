//! The embassy tasks that handle communications between the FSM, CAN, and the
//! GS.

use defmt::todo;
use defmt::*;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::pubsub::WaitResult;
use embassy_sync::signal::Signal;
use embassy_time::Instant;
use embassy_time::Timer;
use embedded_can::Frame;
use embedded_can::Id;
use embedded_can::StandardId;
use lib::config;
use lib::config::Command;
use lib::config::Datatype;
use lib::config::COMMAND_HASH;
use lib::config::CONFIG_HASH;
use lib::config::CRITICAL_DATATYPE_COUNT;
use lib::config::DATA_HASH;
use lib::Datapoint;
use lib::EmergencyType;
use lib::Event;
use lib::EventReceiver;
use lib::EventSender;

use crate::can as can2;
use crate::ethernet;
use crate::ethernet::ticks;
use crate::ethernet::types::PodToGsMessage;

/// Forwards CAN datapoints to the ground station and FSM
#[embassy_executor::task]
pub async fn forward_can_datapoints(
    gs_tx: ethernet::types::PodToGsPublisher<'static>,
    event_sender: EventSender,
    mut can_rx: can2::CanRxSubscriber<'static>,
) {
    loop {
        let msg = can_rx.next_message().await;

        let envelope = match msg {
            WaitResult::Message(envelope) => envelope,
            WaitResult::Lagged(i) => {
                warn!("Lagged {} messages", i);
                continue;
            }
        };
        let id = match envelope.id() {
            Id::Extended(e) => e.as_raw(),
            Id::Standard(s) => s.as_raw() as u32,
        };

        let payload = envelope.payload();
        
        // Manual match from a CAN ID to an FSM event
        let event = match_can_id_to_event(id, payload);
        // Send the event to the FSM 
        if event != Event::NoEvent {
            event_sender.send(event).await;
        }

        // Match a CAN ID to an FSM event using the auto-generated method
        let event = lib::config::event_for_can_2_id(id);
        // Send the event to the FSM
        if event != fsm::Event::NoEvent {
            event_sender.send(event).await;
        }

        // Send the datapoint to the ground station
        lib::config::parse_datapoints_can_2(id, payload, |dp| async move {
            gs_tx.send(PodToGsMessage { dp }).await;
        })
            .await;
    }
}

/// Forwards ground station commands to the FSM and over CAN 
#[embassy_executor::task]
pub async fn forward_gs_commands(
    mut gs_rx: ethernet::types::GsToPodSubscriber<'static>,
    event_sender: EventSender,
    can_tx: can2::CanTxSender<'static>,
) {
    loop {
        let msg = gs_rx.next_message_pure().await;
        trace!("Received message from GS: {:?}", msg);
        
        // Get the command sent and match it to an FSM event
        let command: Command = msg.command;
        let event: Event = match_cmd_to_event(command);

        // Send the event to the FSM
        match event {
            Event::NoEvent => {}
            _ => event_sender.send(event).await,
        }
        
        // Forward the command to the CAN bus
        lib::config::gs_to_can2(command, |frame| can_tx.send(frame)).await;
        
        // TODO: Check the hashes that were being sent here?
    }
}

/// Forwards commands and datapoints from the FSM to the ground station and over
/// CAN
#[embassy_executor::task]
pub async fn forward_fsm_events(
    gs_tx: ethernet::types::PodToGsPublisher<'static>,
    can_tx: can2::CanTxSender<'static>,
    event_receiver: EventReceiver,
) -> ! {
    loop {
        // Get the event from the FSM
        let event = event_receiver.receive().await;

        // Match the event to a CAN envelope and send it
        let envelope = match_event_to_can_envelope(event.clone());
        can_tx.send(envelope).await;

        // Match the event to a GroundStationToPod message and send it
        let message = match_event_to_datapoint(event);
        gs_tx.send(message).await;
    }
}

/// Periodically checks if critical datapoints become stale. If so, send an
/// emergency event to the FSM.
#[embassy_executor::task]
pub async fn check_critical_datapoints(
    mut can_rx: can2::CanRxSubscriber<'static>,
    event_sender: EventSender,
    gs_tx: ethernet::types::PodToGsPublisher<'static>,
    signal: &'static Signal<NoopRawMutex, bool>,
) {
    // Wait for the signal to indicate that you are connected to the ground station
    signal.wait().await;

    // Store the timestamp of the last check.
    let mut last_critical_datapoint_check = Instant::now().as_millis();

    // Store the timestamps for each checked datapoint
    // u64: the last timestamp when it received the datapoint
    // bool: if it was sent to the ground station or not. We don't want to send the
    // same datatypes multiple times to avoid popups infinitely triggering on the
    // ground station. This value resets whenever we received the datapoint
    // again.
    let mut critical_datapoints: [(config::Datatype, u64, bool); CRITICAL_DATATYPE_COUNT] =
        [(config::Datatype::DefaultDatatype, 0, false); CRITICAL_DATATYPE_COUNT];

    loop {
        // Check if the channel for receiving CAN messages is empty
        if !can_rx.is_empty() {
            let can_frame = can_rx.next_message_pure().await;

            let id = match can_frame.id() {
                Id::Standard(s) => s.as_raw() as u32,
                Id::Extended(e) => e.as_raw(),
            };

            // get the datatypes associated with the ID of the received CAN message
            let received_datatypes = lib::config::match_can_to_datatypes(id);

            // Check if the received datatypes are critical
            for datatype in received_datatypes {
                if datatype == Datatype::DefaultDatatype {
                    break;
                }
                if datatype.is_critical() {
                    let mut index = 0;

                    // Update the list of critical datatypes
                    loop {
                        let dtt = critical_datapoints[index];
                        if dtt.0 == datatype || dtt.0 == Datatype::DefaultDatatype {
                            critical_datapoints[index] =
                                (datatype, Instant::now().as_millis(), true);
                            break;
                        }

                        index += 1;
                        if index == CRITICAL_DATATYPE_COUNT {
                            error!("Didn't find critical datatype!!");
                            break;
                        }
                    }
                }
            }
        }

        // The time in milliseconds after which a datatype is considered stale
        let timeout_time: u64 = 1000;

        // Check if the data is stale every 250 milliseconds
        let now = Instant::now().as_millis();
        if now - last_critical_datapoint_check >= 250 {
            last_critical_datapoint_check = now;

            let mut index = 0;

            loop {
                let dtt = critical_datapoints[index];
                if dtt.0 == Datatype::DefaultDatatype {
                    break;
                } else if now - dtt.1 >= timeout_time {
                    // Send a message to the fsm to enter emergency
                    event_sender
                        .send(Event::Emergency {
                            emergency_type: EmergencyType::StaleCriticalDataEmergency,
                        })
                        .await;

                    // Send the ID of the stale datatype to the ground station
                    if dtt.2 {
                        gs_tx
                            .send(PodToGsMessage {
                                dp: Datapoint {
                                    datatype: Datatype::EmergencyStaleCriticalData,
                                    value: dtt.0.to_id() as u64,
                                    timestamp: Instant::now().as_ticks(),
                                },
                            })
                            .await;
                    }

                    // Set the bool value to false to indicate that it shouldn't be sent again.
                    critical_datapoints[index] = (dtt.0, dtt.1, false);
                }
                index += 1;

                if index == CRITICAL_DATATYPE_COUNT {
                    break;
                }
            }
        }

        Timer::after_micros(10).await;
    }
}

/// Matches a CAN ID from the received data to an event for the FSM. This is
/// different to the method in `config.rs` because it returns an event based on
/// the payload or an emergency which also requires the type of emergency.
fn match_can_id_to_event(id: u32, payload: &[u8]) -> Event {
    match id {
        831 => {
            let pressure_low = u16::from_be_bytes([payload[0], payload[1]]);
            if pressure_low < 3500 {
                Event::EbsPressureDeployed
            } else {
                Event::EbsPressureRetracted
            }
        }

        // If it gets a ptc logs message from the powertrain controller with state HV
        // on, send ack to fsm
        1251 if payload[0] == 0 => Event::PTCIdleAck,
        1251 if payload[0] == 2 => Event::HVOnAck,
        1251 if payload[0] == 3 => Event::PTCFailure,

        // Check if the velocity is 0, which means that the pod is not moving (used for
        // transitioning from the braking state to the levitating state)
        826 if i16::from_be_bytes([payload[4], payload[5]]) == 0 => Event::Stopped,

        // Levi FSM update ack
        905 if payload[0] == 2 => Event::LeviOnAck,
        905 if payload[0] == 1 => Event::LeviOffAck,

        // Response from levi to the system check
        906 => {
            if payload[0] == 1 {
                Event::LeviSystemCheckSuccess
            } else {
                error!("Different system check payload for levi: {}", payload[0]);
                Event::LeviSystemCheckFailure
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
pub fn match_event_to_can_envelope(event: Event) -> lib::can::can2::CanEnvelope {
    match event {
        fsm::Event::FSMTransition(state_number) => {
            lib::can::can2::CanEnvelope::new_with_id(Command::FSMUpdate(0).to_id(), &[state_number])
        }
        fsm::Event::Discharge => {
            lib::can::can2::CanEnvelope::new_with_id(Command::StopHV(0).to_id(), &[0])
        }
        _ => {}
    }
}

/// Matches an event from the FSM to a datapoint and payload packed into a
/// PodToGsMessage
pub fn match_event_to_datapoint(event: Event) -> PodToGsMessage {
    match event {
        Event::FSMTransition(transitioned_state) => PodToGsMessage {
            dp: Datapoint::new(
                Datatype::FSMState,
                transitioned_state as u64,
                embassy_time::Instant::now().as_ticks(),
            ),
        },
        Event::TransitionFail(other_state) => PodToGsMessage {
            dp: Datapoint::new(
                Datatype::FSMTransitionFail,
                other_state as u64,
                embassy_time::Instant::now().as_ticks(),
            ),
        },
        Event::Emergency { emergency_type } => PodToGsMessage {
            dp: Datapoint::new(
                Datatype::Emergency,
                (emergency_type as i32 + 1) as u64,
                embassy_time::Instant::now().as_ticks(),
            ),
        },
        Event::LeviSystemCheckFailure => PodToGsMessage {
            dp: Datapoint::new(
                Datatype::LeviSystemCheckFailure,
                0,
                Instant::now().as_ticks(),
            ),
        },
        Event::LeviSystemCheckSuccess => PodToGsMessage {
            dp: Datapoint::new(
                Datatype::LeviSystemCheckSuccess,
                0,
                Instant::now().as_ticks(),
            ),
        },
        Event::Prop1SystemCheckFailure => PodToGsMessage {
            dp: Datapoint::new(
                Datatype::Prop1SystemCheckFailure,
                0,
                Instant::now().as_ticks(),
            ),
        },
        Event::Prop1SystemCheckSuccess => PodToGsMessage {
            dp: Datapoint::new(
                Datatype::Prop1SystemCheckSuccess,
                0,
                Instant::now().as_ticks(),
            ),
        },
        Event::Prop2SystemCheckSuccess => PodToGsMessage {
            dp: Datapoint::new(
                Datatype::Prop2SystemCheckSuccess,
                0,
                Instant::now().as_ticks(),
            ),
        },
        Event::Prop2SystemCheckFailure => PodToGsMessage {
            dp: Datapoint::new(
                Datatype::Prop2SystemCheckFailure,
                0,
                Instant::now().as_ticks(),
            ),
        },
        Event::ResetFSM => PodToGsMessage {
            dp: Datapoint::new(Datatype::ResetFSM, 1, Instant::now().as_ticks()),
        },
        _ => {}
    }
}

/// Matches a `Command` to and FSM `Event`
///
/// -`command`: The command that should be matched to an event
fn match_cmd_to_event(command: Command) -> Event {
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
        Command::ResetLevitation(_) => todo!(),
        Command::ResetPowertrain(_) => todo!(),
        Command::ResetPropulsion(_) => todo!(),

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




// -------------------------------------- OLD TASKS --------------------------------------

























/// Forward the messages received from the GS over CAN2.
///
/// -`gs_rx`: Receiver object for the GS
/// -`can_tx`: The sender object for CAN2
#[embassy_executor::task]
pub async fn forward_gs_to_can2(
    mut gs_rx: ethernet::types::GsToPodSubscriber<'static>,
    gs_tx: ethernet::types::PodToGsPublisher<'static>,
    can_tx: can2::CanTxSender<'static>,
) {
    loop {
        let msg = gs_rx.next_message_pure().await;
        trace!("Received message from GS: {:?}", msg);
        let command = msg.command;

        // Sends the hashes if the GS asks for them
        if let Command::SendHashes(_) = command {
            gs_tx
                .send(PodToGsMessage {
                    dp: Datapoint::new(Datatype::CommandHash, COMMAND_HASH, ticks()),
                })
                .await;
            gs_tx
                .send(PodToGsMessage {
                    dp: Datapoint::new(Datatype::DataHash, DATA_HASH, ticks()),
                })
                .await;
            gs_tx
                .send(PodToGsMessage {
                    dp: Datapoint::new(Datatype::ConfigHash, CONFIG_HASH, ticks()),
                })
                .await;
            gs_tx
                .send(PodToGsMessage {
                    dp: Datapoint::new(Datatype::FrontendHeartbeating, 0, ticks()),
                })
                .await;
        }

        lib::config::gs_to_can2(command, |frame| can_tx.send(frame)).await;
    }
}

/// Forward the messages received on CAN2 to the FSM.
///
/// -`can_rx`: Receiver object for CAN2
/// -`event_sender`: The sender object for FSM events
#[embassy_executor::task]
pub async fn forward_can2_messages_to_fsm(
    mut can_rx: can2::CanRxSubscriber<'static>,
    event_sender: EventSender,
) {
    loop {
        let msg = can_rx.next_message().await;

        let envelope = match msg {
            WaitResult::Message(envelope) => envelope,
            WaitResult::Lagged(i) => {
                warn!("Lagged {} messages", i);
                continue;
            }
        };

        let id = match envelope.id() {
            Id::Extended(e) => e.as_raw(),
            Id::Standard(s) => s.as_raw() as u32,
        };

        let payload = envelope.payload();
        let event = match_can_id_to_event(id, payload);

        if event != Event::NoEvent {
            event_sender.send(event).await;
        }

        let fsm_event = lib::config::event_for_can_2_id(id);

        if fsm_event != fsm::Event::NoEvent {
            event_sender.send(fsm_event).await;
        }
    }
}

/// Forward the messages received on CAN2 to the GS.
///
/// -`can_rx`: Receiver object for CAN2
/// -`gs_tx`: Transmitter object for the GS
#[embassy_executor::task]
pub async fn forward_can2_messages_to_gs(
    mut can_rx: can2::CanRxSubscriber<'static>,
    gs_tx: ethernet::types::PodToGsPublisher<'static>,
) {
    loop {
        

        // Timer::after_micros(10).await;
    }
}

/// Forwards all CAN messages to the ground station for logging
// #[embassy_executor::task]
// pub async fn log_can2_on_gs(
//     gs_tx: ethernet::types::PodToGsPublisher<'static>,
//     mut can_rx: can2::CanRxSubscriber<'static>,
// ) {
//     loop {
//         let can_frame = can_rx.next_message_pure().await;
//         let id = match can_frame.id() {
//             Id::Standard(s) => s.as_raw() as u32,
//             Id::Extended(e) => {
//                 warn!("Received extended CAN ID on can2 -> gs: {}", e.as_raw());
//                 continue;
//             }
//         };
//
//         gs_tx
//             .send(PodToGsMessage {
//                 dp: Datapoint::new(
//                     Datatype::CANLog,
//                     u64::from(id),
//                     embassy_time::Instant::now().as_ticks(),
//                 ),
//             })
//             .await;
//         // Timer::after_millis(50).await;
//     }
// }

/// Only used for testing, should not be run in the final version. Continuously
/// sends a random message over CAN 2.
// #[embassy_executor::task]
// pub async fn send_random_msg_continuously(can_tx: can2::CanTxSender<'static>) {
//     loop {
//         let frame = Frame::new(Id::Standard(StandardId::new(826u16).unwrap()), &[1u8; 6])
//             .expect("Invalid frame");
//
//         can_tx
//             .send(lib::can::can2::CanEnvelope::new_from_frame(frame))
//             .await;
//         info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>SENDING");
//
//         Timer::after_millis(100).await;
//     }
// }

/// Sends a heartbeat to the ground station every 100 ms
#[embassy_executor::task]
pub async fn gs_heartbeat(gs_tx: ethernet::types::PodToGsPublisher<'static>) {
    let mut value = 1;
    // let mut random: u16 = 200;
    loop {
        // info!("Sending heartbeat");
        gs_tx
            .send(PodToGsMessage {
                dp: Datapoint::new(
                    Datatype::FrontendHeartbeating,
                    value,
                    embassy_time::Instant::now().as_ticks(),
                ),
            })
            .await;
        value = (value + 1) % 2;
        Timer::after_millis(50).await;

        // gs_tx
        //     .send(PodToGsMessage {
        //         dp: Datapoint::new(
        //             Datatype::from_id(random),
        //             random as u64,
        //             embassy_time::Instant::now().as_ticks(),
        //         ),
        //     })
        //     .await;
        // random += 1;
    }
}

