//! Tests for the `MainFSM` struct and its methods.
extern crate embassy_stm32;

use crate::commons::traits::Runner;
use crate::fsm::MainFSM;

#[cfg(test)]
#[embassy_executor::task]
async fn run_fsm(fsm: &'static mut MainFSM) {
    fsm.run().await;
}

#[cfg(test)]
#[embedded_test::tests(setup=crate::tests::commons::setup_log())]
mod main_fsm_tests {
    use embassy_executor::Spawner;
    use embassy_sync::blocking_mutex::raw::NoopRawMutex;
    use static_cell::StaticCell;
    use crate::commons::{EmergencyChannel, EventChannel};
    use crate::fsm::{MainFSM, MainStates};
    use embassy_sync::mutex::Mutex;
    use crate::commons::data::Event;
    use crate::tests::commons::Tools;
    use super::run_fsm;

    extern crate embassy_executor;
    extern crate embassy_stm32;

    static EVENT_CHANNEL: StaticCell<EventChannel> = StaticCell::new();
    static EMERGENCY_CHANNEL: StaticCell<EmergencyChannel> = StaticCell::new();
    static STATE_TRACKER: StaticCell<Mutex<NoopRawMutex, MainStates>> = StaticCell::new();
    static FSM: StaticCell<MainFSM> = StaticCell::new();

    #[init]
    async fn setup() -> Tools<MainState> {
        let spawner = Spawner::for_current_executor().await;
        let event_channel = EVENT_CHANNEL.init(EventChannel::new());
        let emergency_channel = EMERGENCY_CHANNEL.init(EmergencyChannel::new());

        let event_publisher = event_channel.publisher().unwrap();
        let emergency_publisher = emergency_channel.publisher().unwrap();

        let state_tracker = STATE_TRACKER.init(Mutex::new(MainStates::SystemCheck));
        let fsm = FSM.init(MainFSM::new(
            spawner,
            state_tracker,
            event_channel,
            emergency_channel,
        ).await);

        spawner.spawn(run_fsm(fsm)).unwrap();

        let tools = Tools::<MainState> {
            event_publisher,
            emergency_publisher,
            state_tracker,
            spawner
        };

        tools;
    }

    #[test]
    async fn test_basic(state: Tools) {
        let event_list:[(Event, MainStates); 4] = [
            (Event::SystemCheckSuccess, MainStates::Idle),
            (Event::Charge, MainStates::Charging),
            (Event::StopCharge, MainStates::Idle),
            (Event::Activate, MainStates::Active),
        ];

        publish_and_check(&event_list, state.event_publisher, state.state_tracker).await;
    }

    #[test]
    async fn test_ignores_others(state: Tools) {
        let event_list:[(Event, MainStates); 6] = [
            (Event::SystemCheckSuccess, MainStates::Idle),
            (Event::PropulsionOff, MainStates::Idle),
            (Event::LevitationOn, MainStates::Idle),
            (Event::Activate, MainStates::Active),
            (Event::SystemCheckSuccess, MainStates::Active),
            (Event::HighVoltageOn, MainStates::Active)
        ];

        publish_and_check(&event_list, state.event_publisher, state.state_tracker).await;
    }
}