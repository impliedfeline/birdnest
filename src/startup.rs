use std::net::TcpListener;

use axum::{routing::get, Extension, Router};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};

use crate::{
    routes::{health_check, pilots},
    types::Cache,
};

pub async fn run(listener: TcpListener, cache: Cache) -> anyhow::Result<()> {
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/pilots", get(pilots))
        .layer(Extension(cache))
        .layer(cors)
        .layer(CompressionLayer::new());

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
