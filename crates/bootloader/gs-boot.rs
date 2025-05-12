use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

const LISTEN_ADDR: &str = "0.0.0.0:4321";
const CHUNK_SIZE: usize = 512;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 20]; 
    stream.read_exact(&mut buf)?;
    let msg = &buf;
    let is_mismatch = &msg[0..13] == b"HASH MISMATCH"; 
    if is_mismatch {
        println!("Pod reports HASH MISMATCH, sending firmware…");
        send_firmware(&mut stream)?;
        println!("Done.");
    } else {
        println!("Pod reports OK, nothing to do.");
    }
    Ok(())
}

fn send_firmware(stream: &mut TcpStream) -> io::Result<()> {
    let mut file = File::open("app.bin")?;
    let mut chunk = [0u8; CHUNK_SIZE];
    loop {
        let n = file.read(&mut chunk)?;
        if n == 0 { break; }
        stream.write_all(&chunk[..n])?;
        if n < CHUNK_SIZE { break; } 
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(LISTEN_ADDR)?;
    println!("Ground-Station listening on {}", LISTEN_ADDR);

    for socket in listener.incoming() {
        match socket {
            Ok(stream) => {
                println!("Pod connected: {}", stream.peer_addr()?);
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}