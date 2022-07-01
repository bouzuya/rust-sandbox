use async_trait::async_trait;
use domain::{aggregate::IssueBlockLinkAggregate, IssueBlockLinkId};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Error)]
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
        issue_block_link_id: &IssueBlockLinkId,
    ) -> Result<Option<IssueBlockLinkAggregate>, IssueBlockLinkRepositoryError>;

    async fn save(
        &self,
        issue_block_link: &IssueBlockLinkAggregate,
    ) -> Result<(), IssueBlockLinkRepositoryError>;
}

pub trait HasIssueBlockLinkRepository {
    type IssueBlockLinkRepository: IssueBlockLinkRepository + Send + Sync;

    fn issue_block_link_repository(&self) -> &Self::IssueBlockLinkRepository;
}
