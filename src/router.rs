use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::compress;

pub async fn serve(addr: SocketAddr) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("log_router listening on {}", addr);

    loop {
        let (mut socket, peer) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = Vec::new();
            let mut chunk = [0u8; 8192];
            loop {
                match socket.read(&mut chunk).await {
                    Ok(0) => break,
                    Ok(n) => buf.extend_from_slice(&chunk[..n]),
                    Err(e) => {
                        tracing::warn!("read error from {}: {}", peer, e);
                        return;
                    }
                }
            }
            if buf.is_empty() {
                return;
            }
            match compress::compress_zstd(&buf, 3) {
                Ok(compressed) => {
                    let header = format!(
                        "OK compressed {} -> {} bytes\n",
                        buf.len(),
                        compressed.len()
                    );
                    let _ = socket.write_all(header.as_bytes()).await;
                    let _ = socket.write_all(&compressed).await;
                }
                Err(e) => {
                    let _ = socket
                        .write_all(format!("ERR {}\n", e).as_bytes())
                        .await;
                }
            }
        });
    }
}
