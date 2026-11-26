use async_trait::async_trait;
use sqlx::Sqlite;

use crate::domain::{
    comment::{Comment, CommentCount, CommentRepository, CommentStatus, NewComment},
    error::RepositoryError,
};
use crate::infrastructure::sqlite::map_sqlx_error;

use super::TransactionRef;

pub struct SqliteCommentRepository<'a> {
    pub transaction: TransactionRef<'a, Sqlite>,
}

#[async_trait]
impl CommentRepository for SqliteCommentRepository<'_> {
    async fn list_by_post(
        &mut self,
        post_slug: &str,
        approved_only: bool,
    ) -> Result<Vec<Comment>, RepositoryError> {
        let rows = if approved_only {
            sqlx::query_as!(
                CommentRow,
                "SELECT id, post_slug, author, content, created_at, status FROM comments \
                 WHERE post_slug = ? AND status = 'approved' ORDER BY created_at ASC",
                post_slug,
            )
            .fetch_all(&mut **self.transaction)
            .await
            .map_err(map_sqlx_error)?
        } else {
            sqlx::query_as!(
                CommentRow,
                "SELECT id, post_slug, author, content, created_at, status FROM comments \
                 WHERE post_slug = ? ORDER BY created_at ASC",
                post_slug,
            )
            .fetch_all(&mut **self.transaction)
            .await
            .map_err(map_sqlx_error)?
        };

        Ok(rows.into_iter().map(Comment::from).collect())
    }

    async fn list(
        &mut self,
        status: Option<CommentStatus>,
    ) -> Result<Vec<Comment>, RepositoryError> {
        let rows = match status {
            Some(status) => {
                sqlx::query_as!(
                    CommentRow,
                    "SELECT id, post_slug, author, content, created_at, status FROM comments \
                     WHERE status = ? ORDER BY created_at DESC, id DESC",
                    status.as_str(),
                )
                .fetch_all(&mut **self.transaction)
                .await
                .map_err(map_sqlx_error)?
            }
            None => {
                sqlx::query_as!(
                    CommentRow,
                    "SELECT id, post_slug, author, content, created_at, status FROM comments \
                     ORDER BY created_at DESC, id DESC",
                )
                .fetch_all(&mut **self.transaction)
                .await
                .map_err(map_sqlx_error)?
            }
        };

        Ok(rows.into_iter().map(Comment::from).collect())
    }

    async fn find(&mut self, id: i64) -> Result<Option<Comment>, RepositoryError> {
        let row = sqlx::query_as!(
            CommentRow,
            "SELECT id, post_slug, author, content, created_at, status FROM comments \
             WHERE id = ? LIMIT 1",
            id,
        )
        .fetch_optional(&mut **self.transaction)
        .await
        .map_err(map_sqlx_error)?;

        Ok(row.map(Comment::from))
    }

    async fn insert(&mut self, comment: &NewComment) -> Result<Comment, RepositoryError> {
        let row = sqlx::query_as!(
            CommentRow,
            "INSERT INTO comments (post_slug, author, content, status) \
             VALUES (?, ?, ?, 'pending') \
             RETURNING id AS \"id!\", post_slug AS \"post_slug!\", author AS \"author!\", \
             content AS \"content!\", created_at AS \"created_at!\", status AS \"status!\"",
            &comment.post_slug,
            &comment.author,
            &comment.content,
        )
        .fetch_one(&mut **self.transaction)
        .await
        .map_err(map_sqlx_error)?;

        Ok(Comment::from(row))
    }

    async fn update_status(
        &mut self,
        id: i64,
        status: CommentStatus,
    ) -> Result<bool, RepositoryError> {
        let result = sqlx::query!(
            "UPDATE comments SET status = ? WHERE id = ?",
            status.as_str(),
            id,
        )
        .execute(&mut **self.transaction)
        .await
        .map_err(map_sqlx_error)?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete(&mut self, id: i64) -> Result<bool, RepositoryError> {
        let result = sqlx::query!("DELETE FROM comments WHERE id = ?", id)
            .execute(&mut **self.transaction)
            .await
            .map_err(map_sqlx_error)?;

        Ok(result.rows_affected() > 0)
    }

    async fn counts_by_post(&mut self) -> Result<Vec<CommentCount>, RepositoryError> {
        let rows = sqlx::query!(
            "SELECT post_slug, \
             SUM(CASE WHEN status = 'approved' THEN 1 ELSE 0 END) AS approved, \
             SUM(CASE WHEN status = 'pending' THEN 1 ELSE 0 END) AS pending \
             FROM comments GROUP BY post_slug",
        )
        .fetch_all(&mut **self.transaction)
        .await
        .map_err(map_sqlx_error)?;

        Ok(rows
            .into_iter()
            .map(|row| CommentCount {
                post_slug: row.post_slug,
                approved: row.approved,
                pending: row.pending,
            })
            .collect())
    }

    async fn count_by_status(&mut self) -> Result<Vec<(CommentStatus, i64)>, RepositoryError> {
        let rows = sqlx::query!(
            "SELECT status, COUNT(*) AS total FROM comments GROUP BY status",
        )
        .fetch_all(&mut **self.transaction)
        .await
        .map_err(map_sqlx_error)?;

        rows.into_iter()
            .map(|row| {
                let status = CommentStatus::parse(&row.status).ok_or_else(|| {
                    RepositoryError::InvalidData(format!("invalid comment status: {}", row.status))
                })?;
                Ok((status, row.total))
            })
            .collect()
    }
}

#[derive(sqlx::FromRow)]
struct CommentRow {
    id: i64,
    post_slug: String,
    author: String,
    content: String,
    created_at: String,
    status: String,
}

impl From<CommentRow> for Comment {
    fn from(row: CommentRow) -> Self {
        let status = CommentStatus::parse(&row.status).unwrap_or(CommentStatus::Pending);

        Self {
            id: row.id,
            post_slug: row.post_slug,
            author: row.author,
            content: row.content,
            created_at: row.created_at,
            status,
        }
    }
}
