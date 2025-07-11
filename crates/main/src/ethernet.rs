//! Contains all the files used for ethernet
use embassy_stm32::eth::Ethernet;
use embassy_stm32::eth::GenericPhy;
use embassy_stm32::peripherals::ETH;

pub mod logic;
pub mod types;

/// an ethernet device peripheral, abstract over the specific PHY used
pub type EthDevice = Ethernet<'static, ETH, GenericPhy>;

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, EthDevice>) -> ! {
    runner.run().await
}
