use async_trait::async_trait;
use sqlx::Sqlite;

use crate::domain::{error::RepositoryError, setting::SettingRepository};
use crate::infrastructure::sqlite::map_sqlx_error;

use super::TransactionRef;

pub struct SqliteSettingRepository<'a> {
    pub transaction: TransactionRef<'a, Sqlite>,
}

#[async_trait]
impl SettingRepository for SqliteSettingRepository<'_> {
    async fn get_all(&mut self) -> Result<Vec<(String, String)>, RepositoryError> {
        let rows = sqlx::query!("SELECT key, value FROM site_settings")
            .fetch_all(&mut **self.transaction)
            .await
            .map_err(map_sqlx_error)?;

        Ok(rows.into_iter().map(|row| (row.key, row.value)).collect())
    }

    async fn set(&mut self, key: &str, value: &str) -> Result<(), RepositoryError> {
        sqlx::query!(
            "INSERT INTO site_settings (key, value) VALUES (?, ?) \
             ON CONFLICT (key) DO UPDATE SET value = excluded.value",
            key,
            value,
        )
        .execute(&mut **self.transaction)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}
