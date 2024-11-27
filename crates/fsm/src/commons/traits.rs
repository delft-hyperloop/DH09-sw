//! This module contains all the traits that the FSMs implement, along with their implementations.

use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::commons::data::{Event, PriorityEventPubSub};

/// Trait implemented by each FSM to run a loop that checks whether events are being sent or not
/// and handle each event.
pub trait Runner {
    /// Returns a mutable reference to the `PriorityEventPubSub` struct stored by the FSM.
    /// Only used for the `run` method of the `Runner` trait.
    fn get_pub_sub_channel(&mut self) -> &mut Arc<PriorityEventPubSub>;

    /// Asynchronous method that handles calls the `handle` method of each FSM.
    /// Only used for the `run` method of the `Runner` trait.
    ///
    /// # Returns:
    /// - boolean value from the `handle` method of each FSM determining whether the fsm should keep running or not.
    async fn handle_events(&mut self, event: Event) -> bool;

    /// Asynchronous method that executes an infinite loop which checks for events
    /// in the `PriorityEventPubSub` and handles them using the `handle_events` method.
    /// It stops the loop if `handle_events` returns 0. This case should only happen if the FSM receives
    /// the `StopSubFSMs` event.
    async fn run(&mut self) {
        loop {
            let event = Arc::get_mut(self.get_pub_sub_channel()).unwrap().poll().await;
            if !self.handle_events(event).await {
                break;
            }
        }
    }
}

/// Trait implemented by each FSM to transition from one state to another.
pub trait Transition<T> {
    /// Callback method executed when entering a new state
    fn entry_method(&mut self) -> fn();

    /// Callback method executed when exiting a state
    fn exit_method(&mut self) -> fn();

    fn set_state(&mut self, new_state: T);

    /// Transitions from one state to the other. Calls the exit method of the old state before
    /// transitioning to the new state and calling the entry method for it.
    fn transition(&mut self, state: T, atomic_bool: Option<&AtomicBool>) {
        // Gets the exit method associated with the current state
        let exit_method = self.exit_method();
        exit_method();

        // Transitions to new state
        self.set_state(state);
        match atomic_bool {
            Some(atomic_bool) => {
                let current = atomic_bool.load(Ordering::Relaxed);
                atomic_bool.store(!current, Ordering::Relaxed);
            },
            None => {}
        }

        // Calls the entry method for the new state
        let entry_method = self.entry_method();
        entry_method();
    }
}
