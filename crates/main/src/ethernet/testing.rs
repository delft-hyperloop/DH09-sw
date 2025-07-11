use core::net::Ipv4Addr;
use core::str::FromStr;
use defmt::{info, warn};
use embassy_net::icmp::PacketMetadata;
use embassy_net::icmp::ping::{PingManager, PingParams};
use embassy_net::Stack;
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn ping_router_and_gs(stack: Stack<'static>) {
    let mut rx_buffer = [0; 256];
    let mut tx_buffer = [0; 256];
    let mut rx_meta = [PacketMetadata::EMPTY];
    let mut tx_meta = [PacketMetadata::EMPTY];

    let mut ping_manager = PingManager::new(stack, &mut rx_meta, &mut rx_buffer, &mut tx_meta, &mut tx_buffer);
    let gs_address = "192.168.0.103";
    let router_address = "192.168.1.1";

    let mut ping_params_gs = PingParams::new(Ipv4Addr::from_str(gs_address).unwrap());
    let mut ping_params_router = PingParams::new(Ipv4Addr::from_str(router_address).unwrap());
    
    ping_params_router.set_payload(b"Hello, router!");
    ping_params_gs.set_payload(b"Hello, gs!");
    
    // signal.wait().await;
    
    loop {
        info!("Pinging gs");
        match ping_manager.ping(&ping_params_gs).await {
            Ok(time) => info!("Ping time of {}: {}ms", gs_address, time.as_millis()),
            Err(ping_error) => warn!("{:?}", ping_error),
        };

        info!("Pinging router");
        match ping_manager.ping(&ping_params_router).await {
            Ok(time) => info!("Ping time of {}: {}ms", router_address, time.as_millis()),
            Err(ping_error) => warn!("{:?}", ping_error),
        };
        
        Timer::after_secs(1).await;
    }
}