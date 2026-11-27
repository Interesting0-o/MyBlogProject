use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("invalid persisted data: {0}")]
    InvalidData(String),
    #[error("repository failure: {0}")]
    Unexpected(String),
}
