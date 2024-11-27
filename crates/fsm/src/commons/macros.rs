//! This module contains macro rules used by the FSMs to either implement traits or declare the FSMs.

/// Returns an instance of the FSM with the specified name, emergency channel, and publisher channel.
///
/// # Parameters
/// - `fsm_name`: The name of the FSM struct being defined.
/// - `emergency_channel`: The channel used for emergency events that need immediate attention.
/// - `publisher_channel`: The channel used for normal events.
///
/// # Returns
/// An instance of the FSM struct, initialized with the provided channels.
#[macro_export]
macro_rules! define_fsm {
    ($fsm_name:ident, $event_channel:expr, $emergency_channel:expr) => {
        $fsm_name::new(PriorityEventPubSub::new(
            $event_channel.publisher().unwrap(),
            $event_channel.subscriber().unwrap(),
            $emergency_channel.publisher().unwrap(),
            $emergency_channel.subscriber().unwrap(),
        ))
    };
}


/// Macro used to implement the `Runner` trait for all FSMs.
///
/// # Parameters
/// - `fsm_struct`: The struct that the trait should be implemented for
#[macro_export]
macro_rules! impl_runner_get_sub_channel {
    ($fsm_struct:ident) => {
        impl Runner for $fsm_struct {
            fn get_pub_sub_channel(&mut self) -> &mut Arc<PriorityEventPubSub> {
                &mut self.priority_event_pub_sub
            }

            async fn handle_events(&mut self, event: Event) -> bool {
                return Self::handle(self, event).await;
            }
        }
    };
}


/// Macro used to implement the `Transition` trait for all FSMs
///
/// # Parameters
/// - `fsm_struct`: The struct that the trait should be implemented for
/// - `fsm_states`: The enum for the states associated with the fsm
#[macro_export]
macro_rules! impl_transition {
    ($fsm_struct:ident, $fsm_states: ident) => {
        impl Transition<$fsm_states> for $fsm_struct {
            fn entry_method(&mut self) -> fn() {
                ENTRY_FUNCTION_MAP[self.state as usize]
            }

            fn exit_method(&mut self) -> fn() {
                EXIT_FUNCTION_MAP[self.state as usize]
            }

            fn set_state(&mut self, new_state: $fsm_states) {
                self.state = new_state;
            }
        }
    };
}
