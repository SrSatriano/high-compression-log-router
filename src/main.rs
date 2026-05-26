mod compress;
mod router;

use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(50051);
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    router::serve(addr).await
}
