use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::pubsub::{PubSubChannel, Publisher, Subscriber, WaitResult};

#[derive(Clone)]
pub enum Event {
    Emergency,
    SystemCheckSuccess,
    Activate,
    Charge,
    StopCharge,
    Operate,        // Event to enter the big operating state
    Demo,
    Cruise,
    Brake,
    ShutDown,
    Quit,
    HighVoltageOn,
    HighVoltageOff,
    Accelerate {
        velocity_profile: u8 // TODO: Change to actual velocity profile
    },
    PropulsionOn,
    PropulsionOff,
    PropulsionRunning,
    LevitationOn,
    LevitationOff,
}

pub trait Runner {
    fn get_sub_channel(&self) -> &SubscriberChannel;

    fn run(&mut self) {
        loop {
            // TODO: Check for emergency first
            if !self.get_sub_channel().is_empty() {
                let event = self.get_sub_channel().try_next_message();
                match event {
                    Some(WaitResult::Message(received_event)) => {
                        self.handle(received_event);
                    }
                    Some(WaitResult::Lagged(amount)) => {
                        // TODO: Problem? This means that the subscriber missed {amount} messages
                    }
                    _ => {
                        // TODO: Problem?
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_runner_get_sub_channel {
    ($fsm_struct:ident) => {
        impl Runner for $fsm_struct {
            fn get_sub_channel(&self) -> &SubscriberChannel {
                &self.sub_channel
            }
        }
    };
}

pub trait Transition<T> {
    fn entry_method(&mut self) -> fn();

    fn exit_method(&mut self) -> fn();

    fn set_state(&mut self, new_state: T);

    fn transition(&mut self, state: T) {
        // Gets the exit method associated with the current state
        let exit_method = self.exit_method();
        exit_method();

        // Transitions to new state
        self.set_state(state);

        // Calls the entry method for the new state
        let entry_method = self.entry_method();
        entry_method();
    }
}

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

// Publishers and subscribers for the channel used to broadcast events to all the FSMs.
// Max 32 events, 6 subscribers, 7 publishers(FSMs and main).
const MAX_EVENTS: usize = 32;
const SUBSCRIBERS: usize = 6;
const PUBLISHERS: usize = 7;
pub type EventChannel = PubSubChannel<NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;
pub type PublisherChannel = Publisher<'static, NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;
pub type SubscriberChannel = Subscriber<'static, NoopRawMutex, Event, MAX_EVENTS, SUBSCRIBERS, PUBLISHERS>;
