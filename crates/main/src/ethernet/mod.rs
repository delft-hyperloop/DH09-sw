//! Contains all the files used for ethernet
use cortex_m::peripheral::SCB;
use defmt::info;
use embassy_net::Ipv4Address;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::Timer;
use lib::config;

use crate::ethernet::types::EthDevice;

pub mod logic;
pub mod types;

/// size in bytes of the TCP incoming buffer, i.e. how many bytes can the
/// network stack receive from the peripheral without us doing socket.recv
pub const RX_BUFFER_SIZE: usize = 8192;
/// same as [`RX_BUFFER_SIZE`] but for transmitting.
/// the main pcb doesn't expect to receive a lot of data, in fact only commands
/// come through tcp, so the transmit buffer is much larger to accomodate
/// outgoing telemetry.
pub const TX_BUFFER_SIZE: usize = 32768;

/// Buffer used by the TCP stack when receiving
pub static mut RX_BUFFER: [u8; RX_BUFFER_SIZE] = [0u8; RX_BUFFER_SIZE];
/// Buffer used by the TCP stack when transmitting
pub static mut TX_BUFFER: [u8; TX_BUFFER_SIZE] = [0u8; TX_BUFFER_SIZE];

/// Boolean used to check if the hashes have been sent or not.
/// Shared between the `timeout_for_sending_hashes` task and the `connect`
/// method from the `GsMaster`
pub static HASH_TIMEOUT_FLAG: Mutex<CriticalSectionRawMutex, bool> = Mutex::new(false);

/// if nothing is sent over tcp for [timeout], send an RST and close the
/// connection. keep alive will send a TCP_KEEP_ALIVE frame every [duration]
/// milliseconds.
pub const SOCKET_KEEP_ALIVE: Duration = Duration::from_millis(40);

/// max references
pub const CAP: usize = 8;
/// max number of subscribers
pub const SUBS: usize = 4;
/// max number of publishers
pub const PUBS: usize = 1;
/// the max number of messages that can be pending in a channel at a given time.
pub const TX_CAP: usize = 1024;

#[embassy_executor::task]
async fn network_stack_task(mut runner: embassy_net::Runner<'static, EthDevice>) -> ! {
    runner.run().await
}

/// get ground station [`Ipv4Address`]
pub fn get_remote_endpoints() -> [(Ipv4Address, u16); config::IP_ADDRESS_COUNT] {
    let ips = config::GS_IP_ADDRESSES;
    ips.map(|x| (Ipv4Address::new(x.0[0], x.0[1], x.0[2], x.0[3]), x.1))
}

/// Task that triggers a hardware reset 1 second after it gets spawned.
#[allow(dead_code)]
#[embassy_executor::task]
async fn hardware_reset_timeout() {
    info!("Starting watchdog for hashes");

    Timer::after_secs(1).await;
    let mut mutex_lock = HASH_TIMEOUT_FLAG.lock().await;
    if !*mutex_lock {
        SCB::sys_reset()
    }
    // how does this ever run if you've reset??
    *mutex_lock = false;
}

/// shorthand for embassy_time ticks
pub fn ticks() -> u64 {
    Instant::now().as_ticks()
}
