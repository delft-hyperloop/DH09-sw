use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::pubsub::{PubSubChannel, Publisher, Subscriber};

pub enum Event {
    Emergency,
    BootSuccess,
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
    fn get_sub_channel(&self) -> EventChannel;

    fn run(&mut self) {
        loop {
            if !self.get_sub_channel().is_empty() {
                let event = self.get_sub_channel().try_receive();
                match event {
                    Ok(received_event) => {
                        self.handle(received_event);
                    }
                    _ => {
                        // TODO: Panic?
                    }
                }
            }
        }
    }
}

pub trait Transition<T> {
    fn entry_method(&self) -> fn();

    fn exit_method(&self) -> fn();

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

// Publishers and subscribers for the channel used to broadcast events to all the FSMs.
// Max 32 events, 6 subscribers, 7 publishers(FSMs and main).
pub type EventChannel = PubSubChannel<NoopRawMutex, Event, 32, 6, 7>;
pub type PublisherChannel = Publisher<'static, NoopRawMutex, Event, 32, 6, 7>;
pub type SubscriberChannel = Subscriber<'static, NoopRawMutex, Event, 32, 6, 7>;
