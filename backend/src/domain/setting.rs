use async_trait::async_trait;

use crate::domain::error::RepositoryError;

#[async_trait]
pub trait SettingRepository: Send {
    async fn get_all(&mut self) -> Result<Vec<(String, String)>, RepositoryError>;

    async fn set(&mut self, key: &str, value: &str) -> Result<(), RepositoryError>;
}
