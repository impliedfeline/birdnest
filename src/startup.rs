use std::net::TcpListener;

use axum::{routing::get, Extension, Router};

use crate::{
    routes::{health_check, pilots},
    types::Cache,
};

pub async fn run(listener: TcpListener, cache: Cache) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/pilots", get(pilots))
        .layer(Extension(cache));

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
