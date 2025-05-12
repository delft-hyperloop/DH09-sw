#![no_std]
#![no_main]

use cortex_m::peripheral::SCB;
use cortex_m_rt::entry;
use panic_halt as _;

use embassy_boot::{BootLoader, BootLoaderConfig};
use embassy_boot_stm32::BlockingFirmwareState;
use embassy_stm32::{
    flash::{Flash, FlashRegion},
    init, 
    Config
};
use smoltcp::iface::{EthernetInterfaceBuilder, NeighborCache, Routes};
use smoltcp::socket::{SocketSet, UdpSocket, UdpSocketBuffer, UdpPacketMetadata};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr};
use sha2::{Digest, Sha256};
use heapless::Vec;


const ACTIVE_FIRMWARE_ADDR: u32 = 0x08020000;   
const DFU_FIRMWARE_ADDR: u32 = 0x08040000;      
const FIRMWARE_MAX_SIZE: usize = 256 * 1024;    

#[entry]
fn main() -> ! {
    const GOLDEN: [u8; 32] = include_bytes!("../golden.sha256")
        .try_into()
        .unwrap();

    let p = init(Config::default());
    
    let mut flash = Flash::new_blocking(p.FLASH);
    
    let active_region = FlashRegion::new(ACTIVE_FIRMWARE_ADDR, FIRMWARE_MAX_SIZE);
    let dfu_region = FlashRegion::new(DFU_FIRMWARE_ADDR, FIRMWARE_MAX_SIZE);
    let state_region = FlashRegion::new(ACTIVE_FIRMWARE_ADDR - 0x4000, 0x4000); 

    let mut hasher = Sha256::new();
    let mut buf = [0u8; 1024];
    let mut offset = 0;
    while offset < active_region.capacity() as u32 {
        let chunk_size = buf.len().min(active_region.capacity() as usize - offset as usize);
        let chunk = &mut buf[..chunk_size];
        
        active_region.read(offset, chunk).unwrap();
        hasher.update(chunk);
        offset += chunk.len() as u32;
    }
    let digest = hasher.finalize();
    let bad_firmware = digest[..] != GOLDEN;

    let eth_dev = p.ETH_MAC;
    static mut NEIGH: [Option<(IpAddress, _)>; 8] = [None; 8];
    let neighbor_cache = NeighborCache::new(unsafe { &mut NEIGH[..] });
    let ip_cidr = IpCidr::new(IpAddress::v4(192,168,1,100), 24);
    let hw_addr = EthernetAddress([0x02,0x00,0x00,0x00,0x00,0x01]);
    let mut iface = EthernetInterfaceBuilder::new(eth_dev)
        .ethernet_addr(hw_addr)
        .neighbor_cache(neighbor_cache)
        .ip_addrs([ip_cidr])
        .routes(Routes::new(unsafe { &mut [] }))
        .finalize();

    static mut RX_META: [UdpPacketMetadata; 4] = [UdpPacketMetadata::EMPTY; 4];
    static mut RX_DATA: [u8; 512] = [0; 512];
    static mut TX_META: [UdpPacketMetadata; 4] = [UdpPacketMetadata::EMPTY; 4];
    static mut TX_DATA: [u8; 512] = [0; 512];
    let mut sockets = SocketSet::new(Vec::new());
    let udp = sockets.add(UdpSocket::new(
        UdpSocketBuffer::new(unsafe { &mut RX_META[..] }, unsafe { &mut RX_DATA[..] }),
        UdpSocketBuffer::new(unsafe { &mut TX_META[..] }, unsafe { &mut TX_DATA[..] }),
    ));
    sockets.get::<UdpSocket>(udp).bind(4321).unwrap();
    iface.poll(&mut sockets, Instant::now()).ok();

    let gs = (IpAddress::v4(192,168,1,10), 4321);
    {
        let mut s = sockets.get::<UdpSocket>(udp);
        let _ = s.send_slice(if bad_firmware { b"HASH MISMATCH" } else { b"FW OK" }, gs);
    }

    if bad_firmware {
        let mut offset = 0;
        let mut writer = dfu_region;
        
        writer.erase().unwrap();

        loop {
            iface.poll(&mut sockets, Instant::now()).unwrap();
            let mut s = sockets.get::<UdpSocket>(udp);
            
            if let Ok((pkt, _)) = s.recv() {
                writer.write(offset, pkt).unwrap();
                
                offset += pkt.len() as u32;
                
                if pkt.len() < 512 { 
                    break; 
                }
            }
        }

        writer.finalize().unwrap();

        let mut new_hasher = Sha256::new();
        let mut verify_offset = 0;
        while verify_offset < offset {
            let chunk_size = buf.len().min((offset - verify_offset) as usize);
            let chunk = &mut buf[..chunk_size];
            
            dfu_region.read(verify_offset, chunk).unwrap();
            new_hasher.update(chunk);
            verify_offset += chunk.len() as u32;
        }
        let new_digest = new_hasher.finalize();


        if new_digest[..] == GOLDEN {
            let config = BootLoaderConfig::from_linkerfile_blocking(&active_region, &dfu_region, &state_region);
            let mut loader = BootLoader::new(config).unwrap();
            loader.run().unwrap(); 
        }
    }

    SCB::sys_reset();
    loop {}
}