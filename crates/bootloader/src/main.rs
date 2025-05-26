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


const ACTIVE_FIRMWARE_ADDR: u32 = 0x08018400;   
const DFU_FIRMWARE_ADDR: u32 = 0x08008400;      
const FIRMWARE_MAX_SIZE: usize = 64 * 1024;    

#[entry]
fn main() -> ! {
    const GOLDEN: [u8; 32] = include_bytes!("../golden.sha256")
        .try_into()
        .unwrap();

    let p = init(Config::default());
    
    // Initialize Ethernet PHY
    let eth_pins = embassy_stm32::eth::EthPins {
        ref_clk: p.PA1,
        md_io: p.PA2,
        md_clk: p.PC1,
        crs: p.PA7,
        tx_en: p.PB11,
        tx_d0: p.PB12,
        tx_d1: p.PB13,
        rx_d0: p.PC4,
        rx_d1: p.PC5,
    };

    let mut flash = Flash::new_blocking(p.FLASH);
    
    let active_region = FlashRegion::new(ACTIVE_FIRMWARE_ADDR, FIRMWARE_MAX_SIZE);
    let dfu_region = FlashRegion::new(DFU_FIRMWARE_ADDR, FIRMWARE_MAX_SIZE);
    let state_region = FlashRegion::new(ACTIVE_FIRMWARE_ADDR - 0x4000, 0x4000); 

    // Verify current firmware
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 1024];
    let mut offset = 0;
    let mut bad_firmware = true;

    while offset < active_region.capacity() as u32 {
        let chunk_size = buf.len().min(active_region.capacity() as usize - offset as usize);
        let chunk = &mut buf[..chunk_size];
        
        if let Ok(()) = active_region.read(offset, chunk) {
            hasher.update(chunk);
            offset += chunk.len() as u32;
        } else {
            break; // Handle read error by assuming bad firmware
        }
    }
    
    let digest = hasher.finalize();
    bad_firmware = digest[..] != GOLDEN;

    // Network setup with timeout
    let eth_dev = p.ETH_MAC;
    static mut NEIGH: [Option<(IpAddress, _)>; 8] = [None; 8];
    let neighbor_cache = NeighborCache::new(unsafe { &mut NEIGH[..] });
    let ip_cidr = IpCidr::new(IpAddress::v4(192,168,1,100), 24);
    let hw_addr = EthernetAddress([0x02,0x00,0x00,0x00,0x00,0x01]);
    
    let mut iface = match EthernetInterfaceBuilder::new(eth_dev)
        .ethernet_addr(hw_addr)
        .neighbor_cache(neighbor_cache)
        .ip_addrs([ip_cidr])
        .routes(Routes::new(unsafe { &mut [] }))
        .finalize() {
            Ok(iface) => iface,
            Err(_) => {
                // If network init fails, check firmware and reset
                if bad_firmware {
                    SCB::sys_reset();
                }
                loop {} // Stay in bootloader if network fails but firmware is good
            }
        };

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
        let mut timeout = 1_000_000; // ~1 second timeout
        
        if let Err(_) = writer.erase() {
            SCB::sys_reset(); // Reset if erase fails
        }

        'update: loop {
            if let Ok(()) = iface.poll(&mut sockets, Instant::now()) {
                let mut s = sockets.get::<UdpSocket>(udp);
                
                if let Ok((pkt, _)) = s.recv() {
                    if let Ok(()) = writer.write(offset, pkt) {
                        offset += pkt.len() as u32;
                        
                        if pkt.len() < 512 { 
                            break 'update;
                        }
                        timeout = 1_000_000; // Reset timeout on successful packet
                    }
                }
            }
            
            timeout -= 1;
            if timeout == 0 {
                SCB::sys_reset(); // Reset on timeout
            }
        }

        // Verify new firmware
        if let Ok(()) = writer.finalize() {
            let mut new_hasher = Sha256::new();
            let mut verify_offset = 0;
            let mut verification_ok = false;

            while verify_offset < offset {
                let chunk_size = buf.len().min((offset - verify_offset) as usize);
                let chunk = &mut buf[..chunk_size];
                
                if let Ok(()) = dfu_region.read(verify_offset, chunk) {
                    new_hasher.update(chunk);
                    verify_offset += chunk.len() as u32;
                } else {
                    break;
                }
            }
            
            let new_digest = new_hasher.finalize();
            verification_ok = new_digest[..] == GOLDEN;

            if verification_ok {
                let config = BootLoaderConfig::from_linkerfile_blocking(&active_region, &dfu_region, &state_region);
                if let Ok(mut loader) = BootLoader::new(config) {
                    let _ = loader.run(); // Attempt to run new firmware
                }
            }
        }
    }

    SCB::sys_reset();
    loop {}
}