use std::{io::IsTerminal, net::SocketAddr, sync::Arc};

use sudo4_api::{
    api::{self, AppState},
    config::CONFIG,
    infrastructure::sqlite::{SqliteUnitOfWorkFactory, connect},
    service::{auth::AuthService, comment::CommentService, post::PostService, setting::SettingService, stats::StatsService},
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

    let factory = Arc::new(SqliteUnitOfWorkFactory::new(pool));

    let state = AppState {
        posts: PostService::new(factory.clone()),
        comments: CommentService::new(factory.clone()),
        settings: SettingService::new(factory.clone()),
        stats: StatsService::new(factory.clone()),
        auth: AuthService::new(),
    };

    let app = api::router(state);
    let address = format!("{}:{}", CONFIG.backend_host, CONFIG.backend_port);
    let address: SocketAddr = address.parse().expect("Invalid BACKEND_HOST/BACKEND_PORT");

    if CONFIG.admin_password_hash.is_none() || CONFIG.admin_jwt_secret.is_none() {
        tracing::warn!(
            "ADMIN_PASSWORD_HASH / ADMIN_JWT_SECRET 未配置，登录功能不可用；请参考 README 配置后再开放后台"
        );
    }

    axum::serve(
        tokio::net::TcpListener::bind(address)
            .await
            .expect("Failed to bind server"),
        app,
    )
    .await
    .expect("Server error");
}
