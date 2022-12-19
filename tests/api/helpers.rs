use std::{
    net::{SocketAddr, TcpListener},
    sync::Arc,
};

use birdnest::{startup::run, types::Cache};
use tokio::sync::RwLock;
use ttl_cache::TtlCache;

pub struct TestApp {
    pub address: SocketAddr,
    pub cache: Cache,
}

pub async fn spawn_app() -> TestApp {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let address = listener.local_addr().unwrap();
    let cache = Arc::new(RwLock::new(TtlCache::new(std::usize::MAX)));
    let app = TestApp {
        address,
        cache: cache.clone(),
    };

    tokio::spawn(async move { run(listener, cache).await.unwrap() });
    app
}
