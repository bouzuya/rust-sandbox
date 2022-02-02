use async_trait::async_trait;
use domain::{
    aggregate::{IssueAggregate, IssueAggregateEvent},
    IssueId,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("IO")]
    IO,
}

#[async_trait]
pub trait IssueRepository {
    async fn find_by_id(
        &self,
        issue_id: &IssueId,
    ) -> Result<Option<IssueAggregate>, RepositoryError>;

    async fn last_created(&self) -> Result<Option<IssueAggregate>, RepositoryError>;

    async fn save(&self, event: IssueAggregateEvent) -> Result<(), RepositoryError>;
}

pub trait HasIssueRepository {
    type IssueRepository: IssueRepository + Send + Sync;

    fn issue_repository(&self) -> &Self::IssueRepository;
}
