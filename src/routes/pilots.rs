use crate::types::{Cache, Violation};
use axum::{Extension, Json};

pub async fn pilots(
    Extension(cache): Extension<Cache>,
) -> Json<Vec<Violation>> {
    let cache_lock = cache.read().await;
    let cache_contents =
        cache_lock.clone().iter().map(|(_, y)| y).cloned().collect();
    Json(cache_contents)
}
