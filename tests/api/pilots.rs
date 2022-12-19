use std::time::Duration;

use birdnest::types::{Information, SerialNumber};

use crate::helpers::spawn_app;

#[tokio::test]
async fn pilots_responds_with_cache() {
    // Arrange
    let app = spawn_app().await;
    let cache_contents: Vec<(SerialNumber, Information)> =
        fake::vec![(SerialNumber, Information); 3];
    let mut cache_lock = app.cache.write().await;
    for (key, value) in cache_contents.clone() {
        cache_lock.insert(key, value, Duration::from_secs(30));
    }
    drop(cache_lock);
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("http://{}/pilots", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    let payload = response
        .json::<Vec<(SerialNumber, Information)>>()
        .await
        .expect("Failed to parse request as JSON");
    assert_eq!(cache_contents, payload);
}
