use std::{io::IsTerminal, net::SocketAddr, sync::Arc};

use sudo4_api::{
    api::{self, AppState},
    config::CONFIG,
    domain::unit_of_work::UnitOfWorkFactory,
    infrastructure::sqlite::{SqliteUnitOfWorkFactory, connect},
    service::auth::LoginGuard,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // 先触发 CONFIG 加载 .env，再按 RUST_LOG 配置日志：
    // 未设置时 debug 构建输出全部日志，release 构建只保留 warning 及以上。
    let _ = &*CONFIG;

    let filter = match std::env::var("RUST_LOG") {
        Ok(value) if !value.trim().is_empty() => EnvFilter::new(value),
        _ => {
            if cfg!(debug_assertions) {
                EnvFilter::new("sudo4_api=trace")
            } else {
                EnvFilter::new("sudo4_api=warn")
            }
        }
    };

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

    let factory: Arc<dyn UnitOfWorkFactory> = Arc::new(SqliteUnitOfWorkFactory::new(pool));

    let state = AppState {
        db: factory,
        auth: LoginGuard::new(),
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
