use std::fmt::Debug;

use async_trait::async_trait;
use domain::{aggregate::IssueAggregate, IssueId, Version};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Error)]
pub enum Error {
    #[error("IO")]
    IO,
    #[error("Unknown: {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait IssueRepository {
    async fn find_by_id(&self, issue_id: &IssueId) -> Result<Option<IssueAggregate>>;

    async fn find_by_id_and_version(
        &self,
        issue_id: &IssueId,
        version: &Version,
    ) -> Result<Option<IssueAggregate>>;

    async fn last_created(&self) -> Result<Option<IssueAggregate>>;

    async fn save(&self, issue: &IssueAggregate) -> Result<()>;
}

pub trait HasIssueRepository {
    type IssueRepository: IssueRepository + Send + Sync;

    fn issue_repository(&self) -> &Self::IssueRepository;
}
