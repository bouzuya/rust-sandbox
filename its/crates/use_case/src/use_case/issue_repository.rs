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

pub trait IssueRepository {
    fn find_by_id(&self, issue_id: &IssueId) -> Result<Option<IssueAggregate>, RepositoryError>;

    fn last_created(&self) -> Result<Option<IssueAggregate>, RepositoryError>;

    fn save(&self, event: IssueAggregateEvent) -> Result<(), RepositoryError>;
}

pub trait HasIssueRepository {
    type IssueRepository: IssueRepository;

    fn issue_repository(&self) -> &Self::IssueRepository;
}
