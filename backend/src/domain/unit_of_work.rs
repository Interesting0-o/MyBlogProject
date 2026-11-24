use async_trait::async_trait;

use crate::domain::error::RepositoryError;
use crate::domain::post::PostRepository;

#[async_trait]
pub trait UnitOfWork: Send {
    fn posts(&mut self) -> Box<dyn PostRepository + '_>;

    async fn commit(self: Box<Self>) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait UnitOfWorkFactory: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, RepositoryError>;
}
