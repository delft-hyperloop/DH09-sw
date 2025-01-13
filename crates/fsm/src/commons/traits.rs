//! This module contains all the traits that the FSMs implement, along with
//! their implementations.

use core::future::Future;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

use crate::commons::data::Event;
use crate::commons::data::PriorityEventPubSub;

/// Trait implemented by each FSM to run a loop that checks whether events are
/// being sent or not and handle each event.
pub trait Runner {
    /// Returns a mutable reference to the `PriorityEventPubSub` struct stored
    /// by the FSM. Only used for the `run` method of the `Runner` trait.
    fn get_pub_sub_channel(&mut self) -> &mut PriorityEventPubSub;

    /// Asynchronous method that handles calls the `handle` method of each FSM.
    /// Only used for the `run` method of the `Runner` trait.
    ///
    /// # Returns:
    /// - future that resolves to the boolean value from the `handle` method of
    ///   each FSM. This determines whether the fsm should keep running or not.
    fn handle_events(&mut self, event: Event) -> impl Future<Output = bool>;

    /// Asynchronous method that executes an infinite loop which checks for
    /// events in the `PriorityEventPubSub` and handles them using the
    /// `handle_events` method. It stops the loop if `handle_events` returns
    /// false. This case should only happen if the FSM receives
    /// the `StopSubFSMs` event.
    fn run(&mut self) -> impl Future<Output = ()> {
        async {
            loop {
                let event = self.get_pub_sub_channel()
                    .get_event()
                    .await;

                defmt::info!("Received event: {:?}", event);

                if !self.handle_events(event).await {
                    break;
                }
            }
        }
    }
}

macro_rules! fsm_transitions {
    (
        $fsm_struct:ident,
        $fsm_state_struct_name:ident,
        $(
            $fsm_state_1:ident => $fsm_state_2:ident ($exit_method:ident, $entry_method:ident),
        )*
    ) => {
    };
}

/// Trait implemented by each FSM to transition from one state to another.
pub trait Transition<T> {
    /// Callback method executed when entering a new state
    fn entry_method(&mut self) -> fn(&mut Self);

    /// Callback method executed when exiting a state
    fn exit_method(&mut self) -> fn(&mut Self);

    /// Sets the new state of the FSM
    async fn set_state(&mut self, new_state: T);

    /// Transitions from one state to the other. Calls the exit method of the
    /// old state before transitioning to the new state and calling the
    /// entry method for it.
    ///
    /// # Parameters:
    /// - `state`: The new state the FSM should transition to
    /// - `atomic_bool`: Option containing the atomic bool used to universally
    ///   keep track of the sub-FSMs state. If `None` is provided, the subsystem
    ///   doesn't change whether it's running or not.
    async fn transition(&mut self, state: T, atomic_bool: Option<&AtomicBool>) {
        // Gets the exit method associated with the current state
        let exit_method = self.exit_method();
        exit_method(self);

        // Transitions to new state
        self.set_state(state).await;
        match atomic_bool {
            Some(atomic_bool) => {
                let current = atomic_bool.load(Ordering::Relaxed);
                atomic_bool.store(!current, Ordering::Relaxed);
            }
            None => {}
        }

        // Calls the entry method for the new state
        let entry_method = self.entry_method();
        entry_method(self);
    }
}
