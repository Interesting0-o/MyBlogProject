use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::Response};

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
