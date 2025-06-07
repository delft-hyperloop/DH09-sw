use std::net::IpAddr;

use anyhow::Result;
use local_ip_address::local_ip;

pub fn configure_gs_ip(mut ip: Vec<[u8; 4]>, port: u16, strict: bool) -> Result<String> {
    if !strict {
        match local_ip()? {
            IpAddr::V4(ipv4) => ip.push(ipv4.octets()),
            IpAddr::V6(_) => {},
        }
    }

    let addr_count = ip.len();

    Ok(format!(
        "
pub static GS_ADDR_COUNT: usize = {addr_count};
pub static GS_IP_PORT: u16 = {port};
pub static mut GS_IP_ADDRESSES: [[u8;4];{addr_count}] = [{}];

#[cfg(target_os = \"none\")]
pub static GS_IP_SOCKETS: [embassy_net::IpEndpoint; {addr_count}] = [{}];

",
        ip.iter()
            .map(|i| format!("[{}u8, {}u8, {}u8, {}u8]", i[0], i[1], i[2], i[3]))
            .collect::<Vec<String>>()
            .join(", "),
        ip.iter()
            .map(|i| format!(
                "embassy_net::IpEndpoint::new(embassy_net::IpAddress::v4({},{},{},{}), {port})",
                i[0], i[1], i[2], i[3]
            ))
            .collect::<Vec<String>>()
            .join(", "),
    ))
}
