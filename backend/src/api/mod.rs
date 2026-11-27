mod admin;
mod comments;
mod error;
mod middleware;
mod posts;
mod settings;
mod state;

use axum::{
    Json, Router, middleware as axum_middleware,
    routing::{get, post},
};
use serde_json::{Value, json};

pub use state::AppState;

use self::middleware::require_admin;

pub fn router(state: AppState) -> Router {
    let public = Router::new()
        .route("/posts", get(posts::list))
        .route("/posts/{slug}", get(posts::get))
        .route(
            "/posts/{slug}/comments",
            get(comments::list_comments).post(comments::create_comment),
        )
        .route("/settings", get(settings::get_settings));

    let protected = Router::new()
        .route("/posts", get(admin::list_posts).post(admin::create_post))
        .route(
            "/posts/{slug}",
            get(admin::get_post)
                .put(admin::update_post)
                .delete(admin::delete_post),
        )
        .route("/comments", get(admin::list_comments))
        .route(
            "/comments/{id}",
            axum::routing::patch(admin::set_comment_status).delete(admin::delete_comment),
        )
        .route(
            "/settings",
            get(admin::get_settings).put(admin::update_settings),
        )
        .route("/stats", get(admin::stats))
        .route("/session", get(admin::session))
        .route_layer(axum_middleware::from_fn_with_state(
            state.clone(),
            require_admin,
        ));

    let admin = Router::new()
        .route("/login", post(admin::login))
        .route("/logout", post(admin::logout))
        .merge(protected);

    Router::new()
        .route("/health", get(health))
        .nest("/api", public)
        .nest("/api/admin", admin)
        .layer(axum_middleware::from_fn(middleware::log_requests))
        .with_state(state)
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
