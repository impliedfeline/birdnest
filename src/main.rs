use std::{net::TcpListener, sync::Arc};

use birdnest::{
    cache_update_worker::run_worker, config::CacheConfig, startup::run,
};
use futures_util::try_join;
use tokio::sync::RwLock;
use ttl_cache::TtlCache;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener =
        TcpListener::bind("0.0.0.0:3000").expect("Failed to bind port");
    let cache = Arc::new(RwLock::new(TtlCache::new(std::usize::MAX)));
    let config = CacheConfig::new()?;

    try_join!(
        run(listener, cache.clone()),
        run_worker(cache.clone(), &config)
    )?;

    Ok(())
}
