use std::sync::Arc;

use thiserror::Error;

use crate::domain::comment::{Comment, CommentStatus, NewComment};
use crate::domain::error::RepositoryError;
use crate::domain::unit_of_work::UnitOfWorkFactory;

#[derive(Debug, Error)]
pub enum CommentServiceError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("post not found")]
    PostNotFound,
    #[error("comment not found")]
    NotFound,
    #[error("invalid comment: {0}")]
    Validation(String),
}

const MAX_AUTHOR_LENGTH: usize = 40;
const MAX_CONTENT_LENGTH: usize = 1000;

#[derive(Clone)]
pub struct CommentService {
    unit_of_work_factory: Arc<dyn UnitOfWorkFactory>,
}

impl CommentService {
    pub fn new(unit_of_work_factory: Arc<dyn UnitOfWorkFactory>) -> Self {
        Self {
            unit_of_work_factory,
        }
    }

    pub async fn list_public(&self, post_slug: &str) -> Result<Vec<Comment>, CommentServiceError> {
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;
        let comments = unit_of_work
            .comments()
            .list_by_post(post_slug.trim(), true)
            .await?;
        unit_of_work.commit().await?;
        Ok(comments)
    }

    pub async fn create(
        &self,
        post_slug: &str,
        author: &str,
        content: &str,
    ) -> Result<Comment, CommentServiceError> {
        let post_slug = post_slug.trim();
        let author = author.trim();
        let content = content.trim();

        if author.is_empty() {
            return Err(CommentServiceError::Validation(
                "author is required".to_string(),
            ));
        }

        if author.chars().count() > MAX_AUTHOR_LENGTH {
            return Err(CommentServiceError::Validation(format!(
                "author must be at most {MAX_AUTHOR_LENGTH} characters"
            )));
        }

        if content.is_empty() {
            return Err(CommentServiceError::Validation(
                "content is required".to_string(),
            ));
        }

        if content.chars().count() > MAX_CONTENT_LENGTH {
            return Err(CommentServiceError::Validation(format!(
                "content must be at most {MAX_CONTENT_LENGTH} characters"
            )));
        }

        let mut unit_of_work = self.unit_of_work_factory.begin().await?;

        let post = unit_of_work
            .posts()
            .find_by_slug(post_slug)
            .await?
            .ok_or(CommentServiceError::PostNotFound)?;

        if post.status != crate::domain::post::PostStatus::Published {
            return Err(CommentServiceError::PostNotFound);
        }

        let comment = unit_of_work
            .comments()
            .insert(&NewComment {
                post_slug: post_slug.to_string(),
                author: author.to_string(),
                content: content.to_string(),
            })
            .await?;

        unit_of_work.commit().await?;
        Ok(comment)
    }

    pub async fn list(
        &self,
        status: Option<CommentStatus>,
    ) -> Result<Vec<Comment>, CommentServiceError> {
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;
        let comments = unit_of_work.comments().list(status).await?;
        unit_of_work.commit().await?;
        Ok(comments)
    }

    pub async fn find(&self, id: i64) -> Result<Option<Comment>, CommentServiceError> {
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;
        let comment = unit_of_work.comments().find(id).await?;
        unit_of_work.commit().await?;
        Ok(comment)
    }

    pub async fn set_status(
        &self,
        id: i64,
        status: CommentStatus,
    ) -> Result<(), CommentServiceError> {
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;

        if !unit_of_work.comments().update_status(id, status).await? {
            return Err(CommentServiceError::NotFound);
        }

        unit_of_work.commit().await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), CommentServiceError> {
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;

        if !unit_of_work.comments().delete(id).await? {
            return Err(CommentServiceError::NotFound);
        }

        unit_of_work.commit().await?;
        Ok(())
    }
}
