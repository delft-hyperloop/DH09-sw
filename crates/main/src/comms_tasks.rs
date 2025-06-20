//! The embassy tasks that handle communications between the FSM, CAN, and the
//! GS.

use defmt::todo;
use defmt::*;
use embassy_stm32::gpio::Output;
use embassy_sync::pubsub::WaitResult;
use embassy_time::Instant;
use embedded_can::Id;
use embedded_can::StandardId;
use embedded_can::Frame;
use lib::config::Command;
use lib::config::Datatype;
use lib::config::COMMAND_HASH;
use lib::config::CONFIG_HASH;
use lib::config::DATA_HASH;
use lib::Datapoint;
use lib::Event;
use lib::EventReceiver;
use lib::EventSender;
use embassy_time::Timer;

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
    mut rearm_output: Output<'static>,
) {
    loop {
        let msg = gs_rx.next_message_pure().await;
        trace!("Received message from GS: {:?}", msg);
        let command: Command = msg.command;
        let event: Event = match_cmd_to_event(command);

        // TODO: Turn off High Voltage in case of emergency!
        match event {
            Event::NoEvent => {}
            Event::EnterDemo => {
                // Pull pin high
                rearm_output.set_high();

                embassy_time::Timer::after_millis(100).await;
                event_sender.send(event).await;

                // Pull pin low
                rearm_output.set_low();
            }
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

        // HV commands
        Command::StartHV(_) => Event::StartPreCharge,
        Command::StopHV(_) => Event::Discharge,

        // Levi commands
        Command::LevitationOn(_) => Event::Levitate,
        Command::LevitationOff(_) => Event::StopLevitating,

        // Propulsion commands
        Command::PropulsionOn(_) => Event::Accelerate,
        Command::PropulsionOff(_) => Event::Cruise,

        // Control commands
        Command::SystemCheck(_) => todo!(),
        Command::Shutdown(_) => Event::ShutDown,
        Command::RearmSDC(_) => Event::EnterDemo,

        // Reset commands
        Command::SystemReset(_) => Event::ResetFSM,
        Command::ResetSenseCon(_) => Event::ResetFSM,
        Command::ResetLevitation(_) => todo!(),
        Command::ResetPowertrain(_) => todo!(),
        Command::ResetPropulsion(_) => todo!(),

        // TODO: Acknowledgements
        // Command::FSM
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
        if let fsm::Event::FSMTransition(state_number) = event {
            can_tx
                .send(lib::can::can2::CanEnvelope::new_with_id(
                    Command::FSMUpdate(0).to_id(),
                    &[state_number],
                ))
                .await
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
        // TODO: Check if data is inside bounds
        let msg = can_rx.next_message().await;

        let envelope = match msg {
            WaitResult::Message(envelope) => envelope,
            WaitResult::Lagged(i) => {
                warn!("Lagged {} messages", i);
                continue;
            }
        };

        let id = match envelope.id() {
            Id::Extended(_e) => todo!("Nuh-uh"),
            Id::Standard(s) => s.as_raw(),
        };

        let fsm_event = lib::config::event_for_can_2_id(id as u32);

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
        let can_frame = can_rx.next_message_pure().await;
        let id = match can_frame.id() {
            Id::Extended(_extended_id) => todo!("Nuh-uh"),
            Id::Standard(id) => id.as_raw(),
        };

        // info!("Received CAN frame with ID: {}", id);

        let data = can_frame.payload();

        lib::config::parse_datapoints_can_2(id as u32, data, |dp| async move {
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
            _ => {}
        }
    }
}

/// Forwards all CAN messages to the groundstation for logging
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
                warn!("Received extended CAN ID on can1->gs: {}", e.as_raw());
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

/// Only used for testing, should not be run in the final version
#[embassy_executor::task]
pub async fn send_random_msg_continuously(can_tx: can2::CanTxSender<'static>) {
    loop {
        let frame = Frame::new(Id::Standard(StandardId::new(826u16).unwrap()), &[1u8; 6]).expect("Invalid frame");

        can_tx
            .send(lib::can::can2::CanEnvelope::new_from_frame(frame))
            .await;
        info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>SENDING");

        Timer::after_millis(100).await;
    }
}