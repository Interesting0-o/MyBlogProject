use async_trait::async_trait;

use crate::domain::error::RepositoryError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub updated_at: Option<String>,
    pub tags: Vec<String>,
    pub reading_minutes: u32,
    pub featured: bool,
    pub content: String,
}

#[async_trait]
pub trait PostRepository: Send {
    async fn list(&mut self) -> Result<Vec<Post>, RepositoryError>;

    async fn find_by_slug(&mut self, slug: &str) -> Result<Option<Post>, RepositoryError>;

    async fn insert(&mut self, post: &Post) -> Result<(), RepositoryError>;

    async fn update(&mut self, post: &Post) -> Result<bool, RepositoryError>;

    async fn delete(&mut self, slug: &str) -> Result<bool, RepositoryError>;
}
