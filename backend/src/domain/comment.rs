use async_trait::async_trait;

use crate::domain::error::RepositoryError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentStatus {
    Pending,
    Approved,
    Rejected,
}

impl CommentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(Self::Pending),
            "approved" => Some(Self::Approved),
            "rejected" => Some(Self::Rejected),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub id: i64,
    pub post_slug: String,
    pub author: String,
    pub content: String,
    pub created_at: String,
    pub status: CommentStatus,
}

#[derive(Debug, Clone)]
pub struct NewComment {
    pub post_slug: String,
    pub author: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct CommentCount {
    pub post_slug: String,
    pub approved: i64,
    pub pending: i64,
}

#[async_trait]
pub trait CommentRepository: Send {
    async fn list_by_post(
        &mut self,
        post_slug: &str,
        approved_only: bool,
    ) -> Result<Vec<Comment>, RepositoryError>;

    async fn list(&mut self, status: Option<CommentStatus>) -> Result<Vec<Comment>, RepositoryError>;

    async fn find(&mut self, id: i64) -> Result<Option<Comment>, RepositoryError>;

    async fn insert(&mut self, comment: &NewComment) -> Result<Comment, RepositoryError>;

    async fn update_status(&mut self, id: i64, status: CommentStatus) -> Result<bool, RepositoryError>;

    async fn delete(&mut self, id: i64) -> Result<bool, RepositoryError>;

    async fn counts_by_post(&mut self) -> Result<Vec<CommentCount>, RepositoryError>;

    async fn count_by_status(&mut self) -> Result<Vec<(CommentStatus, i64)>, RepositoryError>;
}
