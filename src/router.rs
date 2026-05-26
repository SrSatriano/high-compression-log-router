use std::net::SocketAddr;

use tokio::net::TcpListener;

pub async fn serve(addr: SocketAddr) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (mut socket, peer) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0u8; 65536];
            if let Ok(n) = socket.readable().await {
                let _ = n;
            }
            // TODO: tonic gRPC Ingest service
            let _ = (&mut socket, peer, &mut buf);
        });
    }
}
