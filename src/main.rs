mod compress;
mod router;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr: SocketAddr = "0.0.0.0:50051".parse()?;
    println!("log_router listening on {addr} (zstd streaming)");
    router::serve(addr).await
}
