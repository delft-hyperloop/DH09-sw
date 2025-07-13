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
use crate::matching_methods::match_can_id_to_event;
use crate::matching_methods::match_cmd_to_event;
use crate::matching_methods::match_event_to_can_envelope;
use crate::matching_methods::match_event_to_datapoint;

/// Forwards CAN datapoints to the ground station and FSM as datapoints or
/// events
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

/// Forwards ground station commands to the FSM and over CAN as events or
/// commands
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

        if let Command::SendHashes(_) = command {
            event_sender.send(Event::SendHashes).await;
        }

        // Forward the command to the CAN bus
        config::gs_to_can2(command, |frame| can_tx.send(frame)).await;
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
        match envelope {
            Some(envelope) => can_tx.send(envelope).await,
            None => {}
        }

        // Send hashes to the ground station
        if let Event::SendHashes = event {
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
        }

        // Match the event to a GroundStationToPod message and send it
        let message = match_event_to_datapoint(event);
        match message {
            Some(message) => {
                gs_tx
                    .send(PodToGsMessage {
                        dp: Datapoint::new(message.0, message.1, Instant::now().as_ticks()),
                    })
                    .await
            }
            None => {}
        }
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
