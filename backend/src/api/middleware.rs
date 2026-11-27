use std::time::Instant;

use axum::{
    Json,
    extract::{Request, State},
    http::{StatusCode, header::AUTHORIZATION, header::COOKIE},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::service::auth::{self, AuthError};

use super::state::AppState;

pub(super) const ADMIN_COOKIE: &str = "admin_token";

pub(super) async fn log_requests(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let started = Instant::now();
    let response = next.run(request).await;
    let status = response.status();
    let latency_ms = started.elapsed().as_millis();

    tracing::info!(
        method = %method,
        path = %path,
        status = %status,
        latency_ms = %latency_ms,
        "request completed"
    );

    response
}

pub(super) async fn require_admin(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let Some(token) = extract_token(&request) else {
        return unauthorized();
    };

    match auth::verify_token(&token) {
        Ok(_admin) => next.run(request).await,
        Err(AuthError::NotConfigured) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "error": "admin auth is not configured" })),
        )
            .into_response(),
        Err(error) => {
            tracing::debug!(error = %error, "admin token verification failed");
            unauthorized()
        }
    }
}

fn extract_token(request: &Request) -> Option<String> {
    if let Some(value) = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        && let Some(token) = value.strip_prefix("Bearer ")
    {
        let token = token.trim();

        if !token.is_empty() {
            return Some(token.to_string());
        }
    }

    let cookies = request
        .headers()
        .get(COOKIE)
        .and_then(|value| value.to_str().ok())?;

    cookies.split(';').find_map(|pair| {
        let (name, value) = pair.trim().split_once('=')?;

        if name.trim() == ADMIN_COOKIE && !value.trim().is_empty() {
            Some(value.trim().to_string())
        } else {
            None
        }
    })
}

fn unauthorized() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "authentication required" })),
    )
        .into_response()
}
