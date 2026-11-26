use async_trait::async_trait;
use sqlx::Sqlite;

use crate::domain::{
    error::RepositoryError,
    post::{Post, PostRepository, PostStatus},
};
use crate::infrastructure::sqlite::map_sqlx_error;

use super::TransactionRef;

pub struct SqlitePostRepository<'a> {
    pub transaction: TransactionRef<'a, Sqlite>,
}

#[async_trait]
impl PostRepository for SqlitePostRepository<'_> {
    async fn list(&mut self, status: Option<PostStatus>) -> Result<Vec<Post>, RepositoryError> {
        let rows = match status {
            Some(status) => {
                sqlx::query_as!(
                    PostRow,
                    "SELECT slug, title, description, published_at, updated_at, tags, \
                     reading_minutes, featured, status, views, content FROM posts \
                     WHERE status = ? \
                     ORDER BY published_at DESC, slug ASC",
                    status.as_str(),
                )
                .fetch_all(&mut **self.transaction)
                .await
                .map_err(map_sqlx_error)?
            }
            None => {
                sqlx::query_as!(
                    PostRow,
                    "SELECT slug, title, description, published_at, updated_at, tags, \
                     reading_minutes, featured, status, views, content FROM posts \
                     ORDER BY published_at DESC, slug ASC",
                )
                .fetch_all(&mut **self.transaction)
                .await
                .map_err(map_sqlx_error)?
            }
        };

        rows.into_iter().map(Post::try_from).collect()
    }

    async fn find_by_slug(&mut self, slug: &str) -> Result<Option<Post>, RepositoryError> {
        let row = sqlx::query_as!(
            PostRow,
            "SELECT slug, title, description, published_at, updated_at, tags, \
             reading_minutes, featured, status, views, content FROM posts WHERE slug = ? LIMIT 1",
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
             (slug, title, description, published_at, updated_at, tags, reading_minutes, featured, status, views, content) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?)",
            &post.slug,
            &post.title,
            &post.description,
            &post.published_at,
            post.updated_at.as_deref(),
            tags,
            i64::from(post.reading_minutes),
            i64::from(post.featured),
            post.status.as_str(),
            &post.content,
        )
        .execute(&mut **self.transaction)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn update(&mut self, current_slug: &str, post: &Post) -> Result<bool, RepositoryError> {
        let tags = encode_tags(&post.tags)?;

        let result = sqlx::query!(
            "UPDATE posts SET slug = ?, title = ?, description = ?, published_at = ?, updated_at = ?, \
             tags = ?, reading_minutes = ?, featured = ?, status = ?, content = ? WHERE slug = ?",
            &post.slug,
            &post.title,
            &post.description,
            &post.published_at,
            post.updated_at.as_deref(),
            tags,
            i64::from(post.reading_minutes),
            i64::from(post.featured),
            post.status.as_str(),
            &post.content,
            current_slug,
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

    async fn increment_views(&mut self, slug: &str) -> Result<(), RepositoryError> {
        sqlx::query!("UPDATE posts SET views = views + 1 WHERE slug = ?", slug)
            .execute(&mut **self.transaction)
            .await
            .map_err(map_sqlx_error)?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
pub struct PostRow {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub updated_at: Option<String>,
    pub tags: String,
    pub reading_minutes: i64,
    pub featured: i64,
    pub status: String,
    pub views: i64,
    pub content: String,
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

        let views = u32::try_from(row.views).map_err(|_| {
            RepositoryError::InvalidData(format!("invalid views for {}", row.slug))
        })?;

        let tags = serde_json::from_str::<Vec<String>>(&row.tags).map_err(|error| {
            RepositoryError::InvalidData(format!("invalid tags for {}: {error}", row.slug))
        })?;

        let status = PostStatus::parse(&row.status).ok_or_else(|| {
            RepositoryError::InvalidData(format!("invalid status for {}: {}", row.slug, row.status))
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
            status,
            views,
            content: row.content,
        })
    }
}

pub fn encode_tags(tags: &[String]) -> Result<String, RepositoryError> {
    serde_json::to_string(tags)
        .map_err(|error| RepositoryError::InvalidData(format!("could not encode tags: {error}")))
}
