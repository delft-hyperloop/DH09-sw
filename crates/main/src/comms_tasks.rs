//! The embassy tasks that handle communications between the FSM, CAN, and the
//! GS.

use defmt::todo;
use defmt::*;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::pubsub::WaitResult;
use embassy_time::Instant;
use embassy_time::Timer;
use embedded_can::Frame;
use embedded_can::Id;
use embedded_can::StandardId;
use lib::config::{Command, CRITICAL_DATATYPE_COUNT};
use lib::config::Datatype;
use lib::config::COMMAND_HASH;
use lib::config::CONFIG_HASH;
use lib::config::DATA_HASH;
use lib::{config, Datapoint};
use lib::EmergencyType;
use lib::Event;
use lib::EventReceiver;
use lib::EventSender;

use crate::can::can2;
use crate::ethernet;
use crate::ethernet::types::PodToGsMessage;

/// Forward the messages received from the GS to the FSM.
///
/// -`gs_rx`: Receiver object for the GS
/// -`event_sender`: The sender object for FSM events
/// -`rearm_output`: The pin used to rearm the SDC
#[embassy_executor::task]
pub async fn forward_gs_to_fsm(
    mut gs_rx: ethernet::types::GsToPodSubscriber<'static>,
    event_sender: EventSender,
) {
    loop {
        let msg = gs_rx.next_message_pure().await;
        trace!("Received message from GS: {:?}", msg);
        let command: Command = msg.command;
        let event: Event = match_cmd_to_event(command);

        // TODO: Turn off High Voltage in case of emergency!
        match event {
            Event::NoEvent => {}
            _ => event_sender.send(event).await,
        }
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

        _ => Event::NoEvent,
    }
}

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
            fn ticks() -> u64 {
                Instant::now().as_ticks()
            }

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

/// Forward the messages received from the FSM over CAN2.
///
/// -`can_tx`: The sender object for CAN2
/// -`event_receiver`: The receiver object for FSM events
#[embassy_executor::task]
pub async fn forward_fsm_events_to_can2(
    can_tx: can2::CanTxSender<'static>,
    event_receiver: EventReceiver,
) {
    loop {
        let event = event_receiver.receive().await;
        match event {
            fsm::Event::FSMTransition(state_number) => {
                can_tx
                    .send(lib::can::can2::CanEnvelope::new_with_id(
                        Command::FSMUpdate(0).to_id(),
                        &[state_number],
                    ))
                    .await
            }
            fsm::Event::Discharge => {
                can_tx
                    .send(lib::can::can2::CanEnvelope::new_with_id(
                        Command::StopHV(0).to_id(),
                        &[0],
                    ))
                    .await
            }
            _ => {}
        }
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

/// Matches a CAN ID from the received data to an event for the FSM. This is
/// different to the method in `config.rs` because it returns an event based on
/// the payload or an emergency which also requires the type of emergency.
fn match_can_id_to_event(id: u32, payload: &[u8]) -> Event {
    match id {
        // If it gets a ptc logs message from the powertrain controller with state HV
        // on, send ack to fsm
        1251 if payload[0] == 2 => Event::HVOnAck,
        1251 if payload[0] == 0 => Event::PTCIdleAck,

        // Check if the velocity is 0, which means that the pod is not moving (used for
        // transitioning from the braking state to the levitating state)
        826 if i16::from_be_bytes([payload[4], payload[5]]) == 0 => Event::Stopped,

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
        877 => {
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
        876 => {
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

        _ => Event::NoEvent,
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
    // Store the timestamp of the last check.
    let mut lastCriticalDatapointCheck = Instant::now();
    
    // Store the timestamps for each checked datapoint
    let mut criticalDatapoints: [(config::Datatype, u32); CRITICAL_DATATYPE_COUNT] = [(config::Datatype::DefaultDatatype, 0); CRITICAL_DATATYPE_COUNT];
    
    loop {
        let can_frame = can_rx.next_message_pure().await;
        let id = match can_frame.id() {
            Id::Extended(extended_id) => extended_id.as_raw(),
            Id::Standard(id) => id.as_raw() as u32,
        };

        let data = can_frame.payload();
        
        

        // send the datapoint to the ground station
        lib::config::parse_datapoints_can_2(id, data, |dp| async move {
            gs_tx.send(PodToGsMessage { dp }).await;
        })
        .await;

        // Timer::after_micros(10).await;
    }
}

/// Forward the messages received from the FSM to the GS.
///
/// -`gs_tx`: Transmitter object for the GS
/// -`event_receiver`: Receiver object for the FSM
#[embassy_executor::task]
pub async fn forward_fsm_to_gs(
    gs_tx: ethernet::types::PodToGsPublisher<'static>,
    event_receiver: EventReceiver,
) {
    let mut levi_failure = 1;
    let mut levi_success = 1;
    let mut prop1_success = 1;
    let mut prop2_success = 1;
    let mut prop1_failure = 1;
    let mut prop2_failure = 1;
    loop {
        let event = event_receiver.receive().await;
        match event {
            Event::FSMTransition(transitioned_state) => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::FSMState,
                            transitioned_state as u64,
                            embassy_time::Instant::now().as_ticks(),
                        ),
                    })
                    .await
            }
            Event::TransitionFail(other_state) => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::FSMTransitionFail,
                            other_state as u64,
                            embassy_time::Instant::now().as_ticks(),
                        ),
                    })
                    .await
            }
            Event::Emergency { emergency_type } => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::Emergency,
                            (emergency_type as i32 + 1) as u64,
                            embassy_time::Instant::now().as_ticks(),
                        ),
                    })
                    .await
            }
            Event::LeviSystemCheckFailure => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::LeviSystemCheckFailure,
                            levi_failure,
                            Instant::now().as_ticks(),
                        ),
                    })
                    .await;
                levi_failure = levi_failure % 100 + 1;
            }
            Event::LeviSystemCheckSuccess => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::LeviSystemCheckSuccess,
                            levi_success,
                            Instant::now().as_ticks(),
                        ),
                    })
                    .await;
                levi_success = levi_success % 100 + 1;
            }
            Event::Prop1SystemCheckFailure => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::Prop1SystemCheckFailure,
                            prop1_failure,
                            Instant::now().as_ticks(),
                        ),
                    })
                    .await;
                prop1_failure = prop1_failure % 100 + 1;
            }
            Event::Prop1SystemCheckSuccess => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::Prop1SystemCheckSuccess,
                            prop1_success,
                            Instant::now().as_ticks(),
                        ),
                    })
                    .await;
                prop1_success = prop1_success % 100 + 1;
            }
            Event::Prop2SystemCheckSuccess => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::Prop2SystemCheckSuccess,
                            prop2_success,
                            Instant::now().as_ticks(),
                        ),
                    })
                    .await;
                prop2_success = prop2_success % 100 + 1;
            }
            Event::Prop2SystemCheckFailure => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(
                            Datatype::Prop2SystemCheckFailure,
                            0, // TODO: this to all of them
                            Instant::now().as_ticks(),
                        ),
                    })
                    .await;
                prop2_failure = prop2_failure % 100 + 1;
            }
            Event::ResetFSM => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(Datatype::ResetFSM, 1, Instant::now().as_ticks()),
                    })
                    .await;
            }
            _ => {}
        }
    }
}

/// Forwards all CAN messages to the ground station for logging
#[embassy_executor::task]
pub async fn log_can2_on_gs(
    gs_tx: ethernet::types::PodToGsPublisher<'static>,
    mut can_rx: can2::CanRxSubscriber<'static>,
) {
    loop {
        let can_frame = can_rx.next_message_pure().await;
        let id = match can_frame.id() {
            Id::Standard(s) => s.as_raw() as u32,
            Id::Extended(e) => {
                warn!("Received extended CAN ID on can2 -> gs: {}", e.as_raw());
                continue;
            }
        };

        gs_tx
            .send(PodToGsMessage {
                dp: Datapoint::new(
                    Datatype::CANLog,
                    u64::from(id),
                    embassy_time::Instant::now().as_ticks(),
                ),
            })
            .await;
        // Timer::after_millis(50).await;
    }
}

/// Only used for testing, should not be run in the final version. Continuously
/// sends a random message over CAN 2.
#[embassy_executor::task]
pub async fn send_random_msg_continuously(can_tx: can2::CanTxSender<'static>) {
    loop {
        let frame = Frame::new(Id::Standard(StandardId::new(826u16).unwrap()), &[1u8; 6])
            .expect("Invalid frame");

        can_tx
            .send(lib::can::can2::CanEnvelope::new_from_frame(frame))
            .await;
        info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>SENDING");

        Timer::after_millis(100).await;
    }
}

/// Sends a heartbeat to the ground station every 100 ms
#[embassy_executor::task]
pub async fn gs_heartbeat(
    gs_tx: ethernet::types::PodToGsPublisher<'static>
) {
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

        // gs_tx.send(PodToGsMessage {
        //     dp: Datapoint::new(
        //         Datatype::Localization,
        //         random as u64,
        //         Instant::now().as_ticks()
        //     )
        // }).await;
        //
        // random += 1;

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
