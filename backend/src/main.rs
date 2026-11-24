use std::{net::SocketAddr, sync::Arc};

use sudo4_api::{
    api,
    config::CONFIG,
    infrastructure::sqlite::{SqliteUnitOfWorkFactory, connect},
    service::post::PostService,
};

#[tokio::main]
async fn main() {
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
