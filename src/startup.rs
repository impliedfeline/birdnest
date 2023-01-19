use std::net::TcpListener;

use axum::{
    body::{boxed, Body},
    http::{Response, StatusCode},
    routing::get,
    Extension, Router,
};
use tower::ServiceExt;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::{
    routes::{health_check, pilots},
    types::Cache,
};

pub async fn run(listener: TcpListener, cache: Cache) -> anyhow::Result<()> {
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/api/health_check", get(health_check))
        .route("/api/pilots", get(pilots))
        .fallback_service(get(|req| async move {
            match ServeDir::new("frontend/build").oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
        .layer(Extension(cache))
        .layer(cors)
        .layer(CompressionLayer::new());

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
