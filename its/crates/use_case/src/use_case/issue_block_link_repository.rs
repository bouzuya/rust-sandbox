use async_trait::async_trait;
use domain::{
    aggregate::{IssueBlockLinkAggregate, IssueBlockLinkAggregateEvent},
    IssueId,
};

use super::repository_error::RepositoryError;

#[async_trait]
pub trait IssueBlockLinkRepository {
    async fn find_by_id(
        &self,
        issue_id: &IssueId,
    ) -> Result<Option<IssueBlockLinkAggregate>, RepositoryError>;

    async fn save(&self, event: IssueBlockLinkAggregateEvent) -> Result<(), RepositoryError>;
}

pub trait HasIssueBlockLinkRepository {
    type IssueBlockLinkRepository: IssueBlockLinkRepository + Send + Sync;

    fn issue_block_link_repository(&self) -> &Self::IssueBlockLinkRepository;
}
