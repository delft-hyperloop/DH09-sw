//! This module contains the tests for the `HighVoltageFSM` struct and its methods.

extern crate embassy_stm32;

use crate::commons::traits::Runner;
use crate::high_voltage_fsm::HighVoltageFSM;

#[cfg(test)]
#[embassy_executor::task]
async fn run_fsm(fsm: &'static mut HighVoltageFSM) {
    fsm.run().await;
}

#[cfg(test)]
#[embedded_test::tests(setup=crate::tests::commons::setup_log())]
mod high_voltage_fsm_tests {

}