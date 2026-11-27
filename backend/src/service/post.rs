use std::collections::HashSet;
use std::sync::Arc;

use thiserror::Error;

use crate::domain::error::RepositoryError;
use crate::domain::post::Post;
use crate::domain::unit_of_work::UnitOfWorkFactory;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("post not found")]
    NotFound,
    #[error("invalid post: {0}")]
    Validation(String),
    #[error("post already exists: {0}")]
    Conflict(String),
}

#[derive(Clone)]
pub struct PostService {
    unit_of_work_factory: Arc<dyn UnitOfWorkFactory>,
}

impl PostService {
    pub fn new(unit_of_work_factory: Arc<dyn UnitOfWorkFactory>) -> Self {
        Self {
            unit_of_work_factory,
        }
    }

    pub async fn list_posts(&self) -> Result<Vec<Post>, ServiceError> {
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;
        let posts = unit_of_work.posts().list().await?;
        unit_of_work.commit().await?;
        Ok(posts)
    }

    pub async fn get_post(&self, slug: &str) -> Result<Post, ServiceError> {
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;
        let post = unit_of_work.posts().find_by_slug(slug).await?;
        unit_of_work.commit().await?;
        post.ok_or(ServiceError::NotFound)
    }

    pub async fn create_post(&self, post: Post) -> Result<Post, ServiceError> {
        let post = normalize_post(post)?;
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;

        if unit_of_work
            .posts()
            .find_by_slug(&post.slug)
            .await?
            .is_some()
        {
            return Err(ServiceError::Conflict(post.slug));
        }

        unit_of_work.posts().insert(&post).await?;
        unit_of_work.commit().await?;
        Ok(post)
    }

    pub async fn update_post(&self, post: Post) -> Result<Post, ServiceError> {
        let post = normalize_post(post)?;
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;

        if !unit_of_work.posts().update(&post).await? {
            return Err(ServiceError::NotFound);
        }

        unit_of_work.commit().await?;
        Ok(post)
    }

    pub async fn delete_post(&self, slug: &str) -> Result<(), ServiceError> {
        let slug = slug.trim();

        if slug.is_empty() {
            return Err(ServiceError::Validation("slug is required".to_string()));
        }

        let mut unit_of_work = self.unit_of_work_factory.begin().await?;

        if !unit_of_work.posts().delete(slug).await? {
            return Err(ServiceError::NotFound);
        }

        unit_of_work.commit().await?;
        Ok(())
    }
}

fn normalize_post(mut post: Post) -> Result<Post, ServiceError> {
    post.slug = post.slug.trim().to_string();
    post.title = post.title.trim().to_string();
    post.description = post.description.trim().to_string();
    post.published_at = post.published_at.trim().to_string();
    post.updated_at = post
        .updated_at
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    post.tags = normalize_tags(post.tags);

    if post.slug.is_empty() {
        return Err(ServiceError::Validation("slug is required".to_string()));
    }

    if post
        .slug
        .chars()
        .any(|character| matches!(character, '/' | '\\' | '#' | '?'))
    {
        return Err(ServiceError::Validation(
            "slug contains reserved characters".to_string(),
        ));
    }

    if post.title.is_empty() {
        return Err(ServiceError::Validation("title is required".to_string()));
    }

    if post.description.is_empty() {
        return Err(ServiceError::Validation(
            "description is required".to_string(),
        ));
    }

    if post.published_at.is_empty() {
        return Err(ServiceError::Validation(
            "published_at is required".to_string(),
        ));
    }

    if post.reading_minutes == 0 {
        return Err(ServiceError::Validation(
            "reading_minutes must be greater than zero".to_string(),
        ));
    }

    Ok(post)
}

fn normalize_tags(tags: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();

    tags.into_iter()
        .map(|tag| tag.trim().to_string())
        .filter(|tag| !tag.is_empty())
        .filter(|tag| seen.insert(tag.clone()))
        .collect()
}
