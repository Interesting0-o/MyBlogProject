mod config;

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};
use std::net::SocketAddr;

use config::CONFIG;

async fn messages() -> Json<Value> {
    Json(json!({"messages": "sucess!"}))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(messages));
    let address = format!("{}:{}", CONFIG.backend_host, CONFIG.backend_port);
    let address: SocketAddr = address.parse().expect("Invalid BACKEND_HOST/BACKEND_PORT");

    axum::serve(
        tokio::net::TcpListener::bind(address)
            .await
            .expect("Failed to bind server"),
        app,
    )
    .await
    .expect("Server error");
}
