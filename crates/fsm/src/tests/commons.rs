//! Contains all the common structures and methods used by tests.

use core::fmt::Debug;

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use lib::utils::types::EventSender;
use lib::utils::Event;

use crate::States;

#[cfg(test)]
pub fn setup_log() {
    rtt_target::rtt_init_defmt!();
}

/// The tools for the trade
/// Struct used setting up every test in the #[init] method
///
/// `T`: The type of the state to be tracked
pub struct Tools {
    pub(crate) event_sender: EventSender,
    pub(crate) state_tracker: &'static Mutex<NoopRawMutex, States>,
}

impl Tools {
    /// Constructor method for the Tools object
    ///
    /// # Returns:
    /// - Instance of Tools structs
    pub fn new(
        event_sender: EventSender,
        state_tracker: &'static Mutex<NoopRawMutex, States>,
    ) -> Self {
        Self {
            event_sender,
            state_tracker,
        }
    }
}

/// Publishes the events passed in the `event_list` and checks if the FSM
/// interprets them properly by comparing its state with the reference state
/// passed with each event.
///
/// # Parameters:
/// - `event_list`: The list of event-state pairs used to check the FSM
/// - `event_publisher`: The publisher object used to publish regular events on
///   the regular channel
/// - `state_tracker`: The static mutex used to track the state of the FSM. It
///   should be the same one as the one passed to the FSM
pub async fn publish_and_check_events<const N: usize, T: Debug + PartialEq>(
    event_list: &[(Event, T); N],
    event_sender: EventSender,
    state_tracker: &'static Mutex<NoopRawMutex, T>,
) {
    for (event, state) in event_list {
        event_sender.send(*event).await;
        let last_state = *(state_tracker.lock().await);
        loop {
            // Let other tasks execute. Without this, the FSM will never run.
            embassy_futures::yield_now().await;
            let current_state = state_tracker.lock().await;
            if *current_state != last_state || last_state == *state {
                assert_eq!(*current_state, *state);
                break;
            }
        }
    }

    event_sender.send(Event::StopFSM).await;
}
