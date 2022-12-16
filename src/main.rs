use std::net::TcpListener;

use birdnest::startup::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener =
        TcpListener::bind("0.0.0.0:3000").expect("Failed to bind port");
    run(listener).await
}
