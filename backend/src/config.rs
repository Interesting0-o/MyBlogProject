use dotenv::dotenv;
use std::{env, sync::LazyLock};

pub struct Config {
    pub backend_host: String,
    pub backend_port: u16,
    pub database_url: String,
    pub database_name: String,
    pub database_user: String,
    pub database_pwd: String,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    dotenv().ok();
    dotenv::from_filename("../.env").ok();

    Config {
        backend_host: env::var("BACKEND_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        backend_port: env::var("BACKEND_PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(3000),
        database_url: env::var("DATABASE_URL").unwrap_or_default(),
        database_name: env::var("DATABASE_NAME").unwrap_or_default(),
        database_user: env::var("DATABASE_USER").unwrap_or_default(),
        database_pwd: env::var("DATABASE_PWD").unwrap_or_default(),
    }
});
