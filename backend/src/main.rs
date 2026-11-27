use std::{io::IsTerminal, net::SocketAddr, sync::Arc};

use sudo4_api::{
    api,
    config::CONFIG,
    infrastructure::sqlite::{SqliteUnitOfWorkFactory, connect},
    service::post::PostService,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("sudo4_api=info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(std::io::stdout().is_terminal())
        .init();

    let pool = connect(&CONFIG.database_url, &CONFIG.base_dir)
        .await
        .expect("Failed to connect to SQLite");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let service = PostService::new(Arc::new(SqliteUnitOfWorkFactory::new(pool)));
    let app = api::router(service);
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
