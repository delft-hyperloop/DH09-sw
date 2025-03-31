use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind(std::net::SocketAddr::new(
        std::net::IpAddr::from([0, 0, 0, 0]),
        6949,
    ))
    .await
    .expect("failed to bind");
    let (mut socket, _) = listener.accept().await.expect("failed to accept");
    let mut buf = [0; 1024];
    loop {
        let n = socket.read(&mut buf).await.expect("failed to read");
        if n == 0 {
            break;
        }
        println!(
            "Received {} bytes: {}",
            n,
            String::from_utf8_lossy(&buf[..n])
        );
    }
}
