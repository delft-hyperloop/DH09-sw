use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpListener;
use embassy_net::tcp::TcpSocket;
use embassy_net::Ipv4Address;
use embassy_net::Stack;
use embassy_stm32::eth::GenericPhy;
use embassy_stm32::eth::Ethernet;
use embassy_stm32::peripherals::ETH;
use embassy_time::Timer;
use embedded_io_async::Read;
use embedded_io_async::Write;
use static_cell::StaticCell;

const FIRMWARE_UPDATE_PORT: u16 = 4322; 
const CHUNK_SIZE: usize = 512;

#[embassy_executor::task]
async fn firmware_update_server(stack: Stack<'static>) -> ! {
    let mut socket = TcpSocket::new(stack, &mut [0; 1024], &mut [0; 1024]);
    

    socket.set_timeout(Some(embassy_time::Duration::from_secs(30)));
    
    info!("Firmware update server binding to port {}", FIRMWARE_UPDATE_PORT);
    let mut listener = TcpListener::new(stack, &mut socket);
    listener.bind(FIRMWARE_UPDATE_PORT).unwrap();
    
    loop {
        info!("Waiting for firmware update connection");
        
        let (mut socket, remote) = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                error!("Accept error: {:?}", e);
                Timer::after_secs(1).await;
                continue;
            }
        };
        
        info!("Connection from {}", remote);
        

        match handle_client(&mut socket).await {
            Ok(_) => info!("Firmware update completed successfully"),
            Err(e) => error!("Firmware update error: {:?}", e),
        }
        

        socket.close();
        Timer::after_secs(1).await;
    }
}

async fn handle_client(socket: &mut TcpSocket<'_>) -> Result<(), embassy_net::tcp::Error> {
    let mut buf = [0u8; 20];
    

    socket.read_exact(&mut buf).await?;
    
    let is_mismatch = &buf[0..13] == b"HASH MISMATCH";
    
    if is_mismatch {
        info!("Pod reports HASH MISMATCH, sending firmware…");
        send_firmware(socket).await?;
        info!("Firmware sending complete.");
    } else {
        info!("Pod reports OK, nothing to do.");
    }
    
    Ok(())
}

async fn send_firmware(socket: &mut TcpSocket<'_>) -> Result<(), embassy_net::tcp::Error> {
    let firmware_data: &[u8] = include_bytes!("../../target/release/gs-boot");
    
    let mut sent = 0;
    while sent < firmware_data.len() {
        let chunk_end = (sent + CHUNK_SIZE).min(firmware_data.len());
        let chunk = &firmware_data[sent..chunk_end];
        
        socket.write_all(chunk).await?;
        
        sent += chunk.len();
        
        if chunk.len() < CHUNK_SIZE {
            break;
        }
    }
    
    Ok(())
}

pub struct FirmwareUpdateServerInitializer {
    device: Ethernet<'static, ETH, GenericPhy>,
    config: embassy_net::Config,
}

impl FirmwareUpdateServerInitializer {
    pub fn new(device: Ethernet<'static, ETH, GenericPhy>, config: embassy_net::Config) -> Self {
        Self { device, config }
    }

    pub async fn init(self, spawner: Spawner) -> Result<(), embassy_net::Error> {
        let seed = 0xABCD1234; 

        static RESOURCES: StaticCell<embassy_net::StackResources<3>> = StaticCell::new();
        let (stack, runner) = embassy_net::new(
            self.device, 
            self.config, 
            RESOURCES.init(embassy_net::StackResources::new()), 
            seed
        );

        spawner.spawn(eth_task(runner)).unwrap();

        stack.wait_config_up().await;

        spawner.spawn(firmware_update_server(stack)).unwrap();

        Ok(())
    }
}

#[embassy_executor::task]
async fn eth_task(mut runner: embassy_net::Runner<'static, Ethernet<'static, ETH, GenericPhy>>) -> ! {
    runner.run().await
}