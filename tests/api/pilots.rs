use std::{iter::zip, time::Duration};

use birdnest::types::{SerialNumber, Violation};

use crate::helpers::spawn_app;

#[tokio::test]
async fn pilots_responds_with_cache() {
    // Arrange
    let app = spawn_app().await;
    let serial_numbers: Vec<SerialNumber> = fake::vec![SerialNumber; 3];
    let violations: Vec<Violation> = fake::vec![Violation; 3];

    let mut cache_lock = app.cache.write().await;
    for (key, value) in zip(serial_numbers, violations.clone()) {
        cache_lock.insert(key, value, Duration::from_secs(30));
    }
    drop(cache_lock);

    // Act
    let response = reqwest::get(format!("http://{}/api/pilots", app.address))
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    let payload = response
        .json::<Vec<Violation>>()
        .await
        .expect("Failed to parse request as JSON");
    assert_eq!(violations, payload);
}
