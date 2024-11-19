use alloc::sync::Arc;
use crate::commons::{Event, PriorityEventPubSub, Runner, Transition};
use crate::{impl_runner_get_sub_channel, impl_transition};

#[derive(PartialEq, Debug, Clone, Copy)]
pub(super) enum HVStates {
    HighVoltageOff = 0,
    HighVoltageOn,
}

pub(super) struct HighVoltageFSM {
    state: HVStates,
    priority_event_pub_sub: Arc<PriorityEventPubSub>,
    // peripherals: // TODO
}

impl HighVoltageFSM {
    pub fn new(
        priority_event_pub_sub: PriorityEventPubSub,
        //peripherals: // TODO
    ) -> Self {
        Self {
            state: HVStates::HighVoltageOn,
            priority_event_pub_sub: Arc::new(priority_event_pub_sub),
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