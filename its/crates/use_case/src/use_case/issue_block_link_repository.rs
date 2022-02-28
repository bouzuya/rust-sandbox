use async_trait::async_trait;
use domain::{
    aggregate::{IssueBlockLinkAggregate, IssueBlockLinkAggregateEvent},
    IssueId,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IssueBlockLinkRepositoryError {
    #[error("IO")]
    IO,
    #[error("Unknown: {0}")]
    Unknown(String),
}

#[async_trait]
pub trait IssueBlockLinkRepository {
    async fn find_by_id(
        &self,
        issue_id: &IssueId,
    ) -> Result<Option<IssueBlockLinkAggregate>, IssueBlockLinkRepositoryError>;

    async fn save(
        &self,
        event: IssueBlockLinkAggregateEvent,
    ) -> Result<(), IssueBlockLinkRepositoryError>;
}

pub trait HasIssueBlockLinkRepository {
    type IssueBlockLinkRepository: IssueBlockLinkRepository + Send + Sync;

    fn issue_block_link_repository(&self) -> &Self::IssueBlockLinkRepository;
}
