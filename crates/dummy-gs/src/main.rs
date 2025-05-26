//! A tool to send firmware updates to the DFU bootloader via UDP.
use tokio::net::UdpSocket;
use std::fs;
use std::path::PathBuf;
use clap::Parser;
use sha2::{Digest, Sha256};
use std::time::Duration;
use tokio::time::timeout;

const CHUNK_SIZE: usize = 512;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    file: PathBuf,

    #[clap(short, long, value_parser, default_value = "192.168.1.100")]
    target_ip: String,

    #[clap(short, long, value_parser, default_value_t = 4321)]
    target_port: u16,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!(
        "Starting DFU: file={}, target={}:{}",
        args.file.display(),
        args.target_ip,
        args.target_port
    );

    let data = fs::read(&args.file)?;
    println!("Read {} bytes from {}", data.len(), args.file.display());

    // Calculate SHA256 hash
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let firmware_hash = hasher.finalize();
    println!("Firmware SHA256: {:x}", firmware_hash);

    // Create a file with the .sha256 extension containing the hash
    // This assumes the bootloader will look for a file named 'golden.sha256'
    // or that this hash will be manually placed where the bootloader expects it.
    let hash_file_path = args.file.with_extension("sha256");
    fs::write(&hash_file_path, firmware_hash.as_slice())?;
    println!("Wrote raw SHA256 hash to {}", hash_file_path.display());


    let remote_addr = format!("{}:{}", args.target_ip, args.target_port);
    let sock = UdpSocket::bind("0.0.0.0:0").await?; // Bind to any available local port
    sock.connect(&remote_addr).await?;
    println!("Connected to target at {}", remote_addr);

    // Send firmware in chunks
    for (i, chunk) in data.chunks(CHUNK_SIZE).enumerate() {
        match timeout(Duration::from_secs(1), sock.send(chunk)).await {
            Ok(Ok(sent_bytes)) => {
                println!(
                    "Sent chunk {} ({} bytes) of {}",
                    i + 1,
                    sent_bytes,
                    (data.len() + CHUNK_SIZE - 1) / CHUNK_SIZE
                );
            }
            Ok(Err(e)) => {
                eprintln!("Error sending chunk {}: {}", i + 1, e);
                return Err(e);
            }
            Err(_) => {
                eprintln!("Timeout sending chunk {}", i + 1);
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "UDP send timeout"));
            }
        }
        // Small delay between chunks, similar to the python script
        // This might be necessary for the receiver to keep up.
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    println!("Firmware update process completed.");
    // The bootloader should now verify the SHA256 and proceed.
    // We might want to listen for a confirmation from the bootloader.
    // For now, we just send "FW OK" or "HASH MISMATCH".

    // Listen for a response (e.g., "FW OK" or "HASH MISMATCH")
    let mut buf = [0; 1024];
    println!("Waiting for response from bootloader...");
    match timeout(Duration::from_secs(5), sock.recv(&mut buf)).await {
        Ok(Ok(n)) => {
            println!("Received response: {}", String::from_utf8_lossy(&buf[..n]));
        }
        Ok(Err(e)) => {
            eprintln!("Error receiving response: {}", e);
        }
        Err(_) => {
            eprintln!("Timeout waiting for response from bootloader.");
        }
    }

    Ok(())
}
