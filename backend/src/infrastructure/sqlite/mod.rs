pub mod comments;
pub mod posts;
pub mod settings;

use std::{path::Path, str::FromStr, time::Duration};

use async_trait::async_trait;
use sqlx::{
    Sqlite, SqlitePool, Transaction,
    pool::PoolOptions,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
};

use crate::domain::{error::RepositoryError, unit_of_work::{UnitOfWork, UnitOfWorkFactory}};

pub type TransactionRef<'a, DB> = &'a mut Transaction<'static, DB>;

pub async fn connect(database_url: &str, base_dir: &Path) -> Result<SqlitePool, sqlx::Error> {
    let mut options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(5));
    let filename = options.get_filename().to_path_buf();

    if !filename.is_absolute() && filename != Path::new(":memory:") {
        options = options.filename(base_dir.join(&filename));
    }

    let filename = options.get_filename();

    if filename != Path::new(":memory:")
        && let Some(parent) = filename.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(sqlx::Error::Io)?;
    }

    PoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
}

#[derive(Clone)]
pub struct SqliteUnitOfWorkFactory {
    pool: SqlitePool,
}

impl SqliteUnitOfWorkFactory {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitOfWorkFactory for SqliteUnitOfWorkFactory {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, RepositoryError> {
        let transaction = self.pool.begin().await.map_err(map_sqlx_error)?;
        Ok(Box::new(SqliteUnitOfWork { transaction }))
    }
}

struct SqliteUnitOfWork {
    transaction: Transaction<'static, Sqlite>,
}

#[async_trait]
impl UnitOfWork for SqliteUnitOfWork {
    fn posts(&mut self) -> Box<dyn crate::domain::post::PostRepository + '_> {
        Box::new(posts::SqlitePostRepository {
            transaction: &mut self.transaction,
        })
    }

    fn comments(&mut self) -> Box<dyn crate::domain::comment::CommentRepository + '_> {
        Box::new(comments::SqliteCommentRepository {
            transaction: &mut self.transaction,
        })
    }

    fn settings(&mut self) -> Box<dyn crate::domain::setting::SettingRepository + '_> {
        Box::new(settings::SqliteSettingRepository {
            transaction: &mut self.transaction,
        })
    }

    async fn commit(self: Box<Self>) -> Result<(), RepositoryError> {
        let SqliteUnitOfWork { transaction } = *self;
        transaction.commit().await.map_err(map_sqlx_error)
    }
}

pub(crate) fn map_sqlx_error(error: sqlx::Error) -> RepositoryError {
    if let sqlx::Error::Database(database_error) = &error
        && database_error.is_unique_violation()
    {
        return RepositoryError::Conflict(database_error.message().to_string());
    }

    RepositoryError::Unexpected(error.to_string())
}
