mod error;
mod posts;

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

use crate::service::post::PostService;

pub fn router(service: PostService) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/posts", get(posts::list))
        .route("/posts/{slug}", get(posts::get))
        .with_state(service)
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
