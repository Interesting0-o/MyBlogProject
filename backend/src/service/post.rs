use std::collections::HashSet;
use std::sync::Arc;

use thiserror::Error;

use crate::domain::error::RepositoryError;
use crate::domain::post::{Post, PostStatus};
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

    pub async fn list_posts(&self, status: Option<PostStatus>) -> Result<Vec<Post>, ServiceError> {
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;
        let posts = unit_of_work.posts().list(status).await?;
        unit_of_work.commit().await?;
        Ok(posts)
    }

    pub async fn get_post(
        &self,
        slug: &str,
        status: Option<PostStatus>,
    ) -> Result<Post, ServiceError> {
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;
        let post = unit_of_work.posts().find_by_slug(slug).await?;
        unit_of_work.commit().await?;

        let post = post.ok_or(ServiceError::NotFound)?;

        if let Some(required) = status
            && post.status != required
        {
            return Err(ServiceError::NotFound);
        }

        Ok(post)
    }

    pub async fn create_post(&self, post: Post) -> Result<Post, ServiceError> {
        let mut post = normalize_post(post)?;
        post.views = 0;
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

    pub async fn update_post(&self, current_slug: &str, post: Post) -> Result<Post, ServiceError> {
        let mut post = normalize_post(post)?;
        let current_slug = current_slug.trim().to_string();

        if current_slug.is_empty() {
            return Err(ServiceError::Validation("slug is required".to_string()));
        }

        let mut unit_of_work = self.unit_of_work_factory.begin().await?;

        let existing = unit_of_work
            .posts()
            .find_by_slug(&current_slug)
            .await?
            .ok_or(ServiceError::NotFound)?;

        if current_slug != post.slug
            && unit_of_work
                .posts()
                .find_by_slug(&post.slug)
                .await?
                .is_some()
        {
            return Err(ServiceError::Conflict(post.slug));
        }

        post.views = existing.views;
        unit_of_work.posts().update(&current_slug, &post).await?;
        unit_of_work.commit().await?;
        Ok(post)
    }

    pub async fn get_published_post_with_view(&self, slug: &str) -> Result<Post, ServiceError> {
        let mut unit_of_work = self.unit_of_work_factory.begin().await?;
        unit_of_work.posts().increment_views(slug).await?;
        let post = unit_of_work
            .posts()
            .find_by_slug(slug)
            .await?
            .ok_or(ServiceError::NotFound)?;

        if post.status != PostStatus::Published {
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

    if !is_date_string(&post.published_at) {
        return Err(ServiceError::Validation(
            "published_at must be formatted as YYYY-MM-DD".to_string(),
        ));
    }

    if let Some(updated_at) = &post.updated_at
        && !is_date_string(updated_at)
    {
        return Err(ServiceError::Validation(
            "updated_at must be formatted as YYYY-MM-DD".to_string(),
        ));
    }

    if post.reading_minutes == 0 {
        post.reading_minutes = estimate_reading_minutes(&post.content);
    }

    Ok(post)
}

fn is_date_string(value: &str) -> bool {
    let bytes = value.as_bytes();

    if bytes.len() != 10
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || !bytes[..4].iter().all(u8::is_ascii_digit)
        || !bytes[5..7].iter().all(u8::is_ascii_digit)
        || !bytes[8..].iter().all(u8::is_ascii_digit)
    {
        return false;
    }

    let month = value[5..7].parse::<u32>().unwrap_or(0);
    let day = value[8..].parse::<u32>().unwrap_or(0);

    (1..=12).contains(&month) && (1..=31).contains(&day)
}

fn is_cjk(character: char) -> bool {
    matches!(character as u32, 0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xF900..=0xFAFF)
}

fn estimate_reading_minutes(content: &str) -> u32 {
    let mut cjk_characters = 0u32;
    let mut latin_words = 0u32;
    let mut current_word_length = 0usize;

    for character in content.chars() {
        if is_cjk(character) {
            cjk_characters += 1;

            if current_word_length > 0 {
                latin_words += 1;
                current_word_length = 0;
            }
        } else if character.is_alphanumeric() {
            current_word_length += 1;
        } else if current_word_length > 0 {
            latin_words += 1;
            current_word_length = 0;
        }
    }

    if current_word_length > 0 {
        latin_words += 1;
    }

    let minutes = f64::from(cjk_characters) / 400.0 + f64::from(latin_words) / 200.0;
    (minutes.ceil() as u32).max(1)
}

fn normalize_tags(tags: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();

    tags.into_iter()
        .map(|tag| tag.trim().to_string())
        .filter(|tag| !tag.is_empty())
        .filter(|tag| seen.insert(tag.clone()))
        .collect()
}
