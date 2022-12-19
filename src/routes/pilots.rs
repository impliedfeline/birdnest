use crate::types::{Cache, Information, SerialNumber};
use axum::{Extension, Json};

pub async fn pilots(
    Extension(cache): Extension<Cache>,
) -> Json<Vec<(SerialNumber, Information)>> {
    let cache_lock = cache.read().await;
    let cache_contents = cache_lock
        .clone()
        .iter()
        .map(|(x, y)| (x.clone(), y.clone()))
        .collect();
    Json(cache_contents)
}
