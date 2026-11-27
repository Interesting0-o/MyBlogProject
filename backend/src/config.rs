use dotenv::dotenv;
use std::{
    env,
    path::{Path, PathBuf},
    sync::LazyLock,
};

pub struct Config {
    pub backend_host: String,
    pub backend_port: u16,
    pub database_url: String,
    pub base_dir: PathBuf,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    dotenv().ok();
    dotenv::from_path(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.env")).ok();

    Config {
        backend_host: env::var("BACKEND_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        backend_port: env::var("BACKEND_PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(3000),
        database_url: env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://resource/sudo4.db".to_string()),
        base_dir: discover_base_dir(),
    }
});

fn discover_base_dir() -> PathBuf {
    if let Ok(current_dir) = env::current_dir()
        && current_dir.join(".env").exists()
    {
        return current_dir;
    }

    if let Some(project_dir) = Path::new(env!("CARGO_MANIFEST_DIR")).parent()
        && project_dir.join(".env").exists()
    {
        return project_dir.to_path_buf();
    }

    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
