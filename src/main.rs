use std::{net::TcpListener, sync::Arc};

use birdnest::startup::run;
use tokio::sync::RwLock;
use ttl_cache::TtlCache;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener =
        TcpListener::bind("0.0.0.0:3000").expect("Failed to bind port");
    let cache = Arc::new(RwLock::new(TtlCache::new(std::usize::MAX)));
    run(listener, cache).await
}
