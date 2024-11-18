use crate::commons::{Event, PublisherChannel, Runner, SubscriberChannel, Transition};
use crate::{impl_runner_get_sub_channel, impl_transition};

#[derive(PartialEq, Debug, Clone, Copy)]
pub(super) enum HVStates {
    HighVoltageOff = 0,
    HighVoltageOn,
}

pub(super) struct HighVoltageFSM {
    state: HVStates,
    // peripherals: // TODO
    pub_channel: PublisherChannel,
    pub sub_channel: SubscriberChannel,
}

impl HighVoltageFSM {
    pub fn new(
        //peripherals: // TODO
        pub_channel: PublisherChannel,
        sub_channel: SubscriberChannel,
    ) -> Self {
        Self {
            state: HVStates::HighVoltageOn,
            pub_channel,
            sub_channel,
        }
    }

    fn handle(&mut self, event: Event) {
        match (&self.state, event) {
            (_, Event::Emergency) => {
                // TODO: Check Levi and Propulsion and send command to turn off
            }
            (HVStates::HighVoltageOn, Event::HighVoltageOff) => self.transition(HVStates::HighVoltageOff),
            (HVStates::HighVoltageOff, Event::HighVoltageOn) => self.transition(HVStates::HighVoltageOn),
            _ => {}
        }
    }

    pub(super) fn get_state(&self) -> &HVStates {
        &self.state
    }
}

impl_runner_get_sub_channel!(HighVoltageFSM);
impl_transition!(HighVoltageFSM, HVStates);

static ENTRY_FUNCTION_MAP: [fn(); 2] = [
    enter_high_voltage_off,
    enter_high_voltage_on
];

static EXIT_FUNCTION_MAP: [fn(); 2] = [
    || (),
    || (),
];

fn enter_high_voltage_on() {
    // TODO: Send CAN command to turn on high voltage
}

fn enter_high_voltage_off() {
    // TODO: Send CAN command to turn off high voltage
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn setup() -> (Spawner, EventChannel) {
//         let mut spawner = ;
//         let mut event_channel = EventChannel::new();
//         {spawner, event_channel}
//     }
//
//     #[test]
//     fn test_basic_transitions() {
//         let mut { spawner, event_channel } = setup();
//         let mut fsm = HighVoltageFSM::new(spawner, event_channel);
//
//         fsm.run();
//
//
//     }
//
//     #[test]
//     fn test_multiple_events() {
//         // TODO
//     }
//
//     #[test]
//     fn test_invalid_event_order() {
//         // TODO
//     }
// }