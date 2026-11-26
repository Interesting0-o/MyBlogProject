use async_trait::async_trait;

use crate::domain::error::RepositoryError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostStatus {
    Draft,
    Published,
}

impl PostStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Published => "published",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "draft" => Some(Self::Draft),
            "published" => Some(Self::Published),
            _ => None,
        }
    }
}

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
    pub status: PostStatus,
    pub views: u32,
    pub content: String,
}

#[async_trait]
pub trait PostRepository: Send {
    async fn list(&mut self, status: Option<PostStatus>) -> Result<Vec<Post>, RepositoryError>;

    async fn find_by_slug(&mut self, slug: &str) -> Result<Option<Post>, RepositoryError>;

    async fn insert(&mut self, post: &Post) -> Result<(), RepositoryError>;

    async fn update(&mut self, current_slug: &str, post: &Post) -> Result<bool, RepositoryError>;

    async fn delete(&mut self, slug: &str) -> Result<bool, RepositoryError>;

    async fn increment_views(&mut self, slug: &str) -> Result<(), RepositoryError>;
}
