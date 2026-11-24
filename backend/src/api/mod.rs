mod error;
mod middleware;
mod posts;

use axum::{Json, Router, middleware as axum_middleware, routing::get};
use serde_json::{Value, json};

use crate::service::post::PostService;

pub fn router(service: PostService) -> Router {
    let posts = Router::new()
        .route("/posts", get(posts::list))
        .route("/posts/{slug}", get(posts::get));

    Router::new()
        .route("/health", get(health))
        .nest("/api", posts)
        .layer(axum_middleware::from_fn(middleware::log_requests))
        .with_state(service)
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
