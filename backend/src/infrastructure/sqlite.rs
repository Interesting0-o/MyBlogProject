use std::{path::Path, str::FromStr, time::Duration};

use async_trait::async_trait;
use sqlx::{
    Sqlite, SqlitePool, Transaction,
    pool::PoolOptions,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
};

use crate::domain::{
    error::RepositoryError,
    post::{Post, PostRepository},
    unit_of_work::{UnitOfWork, UnitOfWorkFactory},
};

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
    fn posts(&mut self) -> Box<dyn PostRepository + '_> {
        Box::new(SqlitePostRepository {
            transaction: &mut self.transaction,
        })
    }

    async fn commit(self: Box<Self>) -> Result<(), RepositoryError> {
        let SqliteUnitOfWork { transaction } = *self;
        transaction.commit().await.map_err(map_sqlx_error)
    }
}

struct SqlitePostRepository<'a> {
    transaction: &'a mut Transaction<'static, Sqlite>,
}

#[async_trait]
impl PostRepository for SqlitePostRepository<'_> {
    async fn list(&mut self) -> Result<Vec<Post>, RepositoryError> {
        let rows = sqlx::query_as!(
            PostRow,
            "SELECT slug, title, description, published_at, updated_at, tags, \
             reading_minutes, featured, content FROM posts \
             ORDER BY published_at DESC, slug ASC",
        )
        .fetch_all(&mut **self.transaction)
        .await
        .map_err(map_sqlx_error)?;

        rows.into_iter().map(Post::try_from).collect()
    }

    async fn find_by_slug(&mut self, slug: &str) -> Result<Option<Post>, RepositoryError> {
        let row = sqlx::query_as!(
            PostRow,
            "SELECT slug, title, description, published_at, updated_at, tags, \
             reading_minutes, featured, content FROM posts WHERE slug = ? LIMIT 1",
            slug,
        )
        .fetch_optional(&mut **self.transaction)
        .await
        .map_err(map_sqlx_error)?;

        row.map(Post::try_from).transpose()
    }

    async fn insert(&mut self, post: &Post) -> Result<(), RepositoryError> {
        let tags = encode_tags(&post.tags)?;

        sqlx::query!(
            "INSERT INTO posts \
             (slug, title, description, published_at, updated_at, tags, reading_minutes, featured, content) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            &post.slug,
            &post.title,
            &post.description,
            &post.published_at,
            post.updated_at.as_deref(),
            tags,
            i64::from(post.reading_minutes),
            i64::from(post.featured),
            &post.content,
        )
        .execute(&mut **self.transaction)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn update(&mut self, post: &Post) -> Result<bool, RepositoryError> {
        let tags = encode_tags(&post.tags)?;

        let result = sqlx::query!(
            "UPDATE posts SET title = ?, description = ?, published_at = ?, updated_at = ?, \
             tags = ?, reading_minutes = ?, featured = ?, content = ? WHERE slug = ?",
            &post.title,
            &post.description,
            &post.published_at,
            post.updated_at.as_deref(),
            tags,
            i64::from(post.reading_minutes),
            i64::from(post.featured),
            &post.content,
            &post.slug,
        )
        .execute(&mut **self.transaction)
        .await
        .map_err(map_sqlx_error)?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete(&mut self, slug: &str) -> Result<bool, RepositoryError> {
        let result = sqlx::query!("DELETE FROM posts WHERE slug = ?", slug)
            .execute(&mut **self.transaction)
            .await
            .map_err(map_sqlx_error)?;

        Ok(result.rows_affected() > 0)
    }
}

#[derive(sqlx::FromRow)]
struct PostRow {
    slug: String,
    title: String,
    description: String,
    published_at: String,
    updated_at: Option<String>,
    tags: String,
    reading_minutes: i64,
    featured: i64,
    content: String,
}

impl TryFrom<PostRow> for Post {
    type Error = RepositoryError;

    fn try_from(row: PostRow) -> Result<Self, Self::Error> {
        let reading_minutes = u32::try_from(row.reading_minutes).map_err(|_| {
            RepositoryError::InvalidData(format!("invalid reading_minutes for {}", row.slug))
        })?;

        if reading_minutes == 0 {
            return Err(RepositoryError::InvalidData(format!(
                "reading_minutes must be greater than zero for {}",
                row.slug
            )));
        }

        let tags = serde_json::from_str::<Vec<String>>(&row.tags).map_err(|error| {
            RepositoryError::InvalidData(format!("invalid tags for {}: {error}", row.slug))
        })?;

        Ok(Self {
            slug: row.slug,
            title: row.title,
            description: row.description,
            published_at: row.published_at,
            updated_at: row.updated_at,
            tags,
            reading_minutes,
            featured: row.featured != 0,
            content: row.content,
        })
    }
}

fn encode_tags(tags: &[String]) -> Result<String, RepositoryError> {
    serde_json::to_string(tags)
        .map_err(|error| RepositoryError::InvalidData(format!("could not encode tags: {error}")))
}

fn map_sqlx_error(error: sqlx::Error) -> RepositoryError {
    if let sqlx::Error::Database(database_error) = &error
        && database_error.is_unique_violation()
    {
        return RepositoryError::Conflict(database_error.message().to_string());
    }

    RepositoryError::Unexpected(error.to_string())
}
