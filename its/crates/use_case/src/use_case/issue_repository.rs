use async_trait::async_trait;
use domain::{
    aggregate::{IssueAggregate, IssueAggregateEvent},
    IssueId,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IssueRepositoryError {
    #[error("IO")]
    IO,
    #[error("Unknown: {0}")]
    Unknown(String),
}

#[async_trait]
pub trait IssueRepository {
    async fn find_by_id(
        &self,
        issue_id: &IssueId,
    ) -> Result<Option<IssueAggregate>, IssueRepositoryError>;

    async fn last_created(&self) -> Result<Option<IssueAggregate>, IssueRepositoryError>;

    async fn save(&self, event: IssueAggregateEvent) -> Result<(), IssueRepositoryError>;
}

pub trait HasIssueRepository {
    type IssueRepository: IssueRepository + Send + Sync;

    fn issue_repository(&self) -> &Self::IssueRepository;
}
