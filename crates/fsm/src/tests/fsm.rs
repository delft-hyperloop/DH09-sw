//! Tests for the `MainFSM` struct and its methods.
extern crate embassy_stm32;

use crate::fsm::FSM;

#[cfg(test)]
#[embassy_executor::task]
async fn run_fsm(fsm: &'static mut FSM) {
    fsm.run().await;
}

#[cfg(test)]
#[embedded_test::tests(setup=crate::tests::commons::setup_log())]
mod main_fsm_tests {
    use embassy_executor::Spawner;
    use embassy_sync::blocking_mutex::raw::NoopRawMutex;
    use embassy_sync::mutex::Mutex;
    use static_cell::StaticCell;

    use super::run_fsm;
    use crate::tests::commons::publish_and_check_events;
    use crate::tests::commons::Tools;
    use crate::utils::data::EmergencyType::GeneralEmergency;
    use crate::utils::types::EventChannel;
    use crate::utils::types::EventReceiver;
    use crate::utils::types::EventSender;
    use crate::utils::Event;
    use crate::States;
    use crate::FSM;

    extern crate embassy_executor;
    extern crate embassy_stm32;

    static EVENT_CHANNEL: StaticCell<EventChannel> = StaticCell::new();
    static FSM_CELL: StaticCell<FSM> = StaticCell::new();
    static STATE_TRACKER: StaticCell<Mutex<NoopRawMutex, States>> = StaticCell::new();

    #[init]
    async fn setup() -> Tools {
        let spawner = Spawner::for_current_executor().await;
        let event_channel = EVENT_CHANNEL.init(EventChannel::new());

        let event_sender: EventSender = EventSender(event_channel.sender());
        let event_receiver: EventReceiver = EventReceiver(event_channel.receiver());
        let state_tracker = STATE_TRACKER.init(Mutex::new(States::Boot));

        let fsm = FSM_CELL.init(FSM::new(&event_receiver, state_tracker).await);

        spawner.spawn(run_fsm(fsm)).unwrap();

        Tools::new(event_sender, state_tracker)
    }

    #[test]
    async fn test_normal_run(state: Tools) {
        let event_list: [(Event, States); 14] = [
            (Event::ConnectToGS, States::ConnectedToGS),
            (Event::StartSystemCheck, States::SystemCheck),
            (Event::SystemCheckSuccess, States::Idle),
            (Event::StartPreCharge, States::PreCharge),
            (Event::Activate, States::Active),
            (Event::Charge, States::Charging),
            (Event::StopCharge, States::Active),
            (Event::EnterDemo, States::Demo),
            (Event::Levitate, States::Levitating),
            (Event::Accelerate, States::Accelerating),
            (Event::Cruise, States::Cruising),
            (Event::Brake, States::Braking),
            (Event::Stopped, States::Levitating),
            (Event::Discharge, States::Discharge),
        ];

        publish_and_check_events(&event_list, state.event_sender, state.state_tracker).await;
    }

    #[test]
    async fn test_ignores_others(state: Tools) {
        let event_list: [(Event, States); 13] = [
            (Event::ConnectToGS, States::ConnectedToGS),
            (Event::SystemCheckSuccess, States::ConnectedToGS),
            (Event::StartPreCharge, States::ConnectedToGS),
            (Event::Activate, States::ConnectedToGS),
            (Event::Charge, States::ConnectedToGS),
            (Event::StopCharge, States::ConnectedToGS),
            (Event::EnterDemo, States::ConnectedToGS),
            (Event::Levitate, States::ConnectedToGS),
            (Event::Accelerate, States::ConnectedToGS),
            (Event::Cruise, States::ConnectedToGS),
            (Event::Brake, States::ConnectedToGS),
            (Event::Stopped, States::ConnectedToGS),
            (Event::Discharge, States::ConnectedToGS),
        ];

        publish_and_check_events(&event_list, state.event_sender, state.state_tracker).await;
    }

    // #[test]
    // async fn test_emergency(state: Tools) {
    //     let event_list: [(Event, States); 15] = [
    //         (Event::ConnectToGS, States::ConnectedToGS),
    //         (Event::StartSystemCheck, States::SystemCheck),
    //         (Event::SystemCheckSuccess, States::Idle),
    //         (Event::StartPreCharge, States::PreCharge),
    //         (Event::Activate, States::Active),
    //         (Event::Charge, States::Charging),
    //         (Event::StopCharge, States::Active),
    //         (Event::EnterDemo, States::Demo),
    //         (Event::Levitate, States::Levitating),
    //         (Event::Accelerate, States::Accelerating),
    //         (Event::Cruise, States::Cruising),
    //         (Event::Brake, States::Braking),
    //         (Event::Stopped, States::Levitating),
    //         (Event::Discharge, States::Discharge),
    //         (
    //             Event::Emergency {
    //                 emergency_type: GeneralEmergency,
    //             },
    //             States::Fault,
    //         ),
    //     ];
    //
    //     publish_and_check(&event_list, state.event_publisher,
    // state.state_tracker).await; }
}
