use std::net::{SocketAddr, TcpListener};

use birdnest::startup::run;

async fn spawn_app() -> SocketAddr {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let address = listener.local_addr().unwrap();

    tokio::spawn(async move { run(listener).await.unwrap() });
    address
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("http://{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
