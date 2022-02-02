use async_trait::async_trait;
use domain::{
    aggregate::{IssueAggregate, IssueAggregateEvent},
    IssueId,
};

use use_case::{IssueRepository, RepositoryError};

#[derive(Debug, Default)]
pub struct SqliteIssueRepository {}

#[async_trait]
impl IssueRepository for SqliteIssueRepository {
    async fn find_by_id(
        &self,
        issue_id: &IssueId,
    ) -> Result<Option<IssueAggregate>, RepositoryError> {
        todo!()
    }

    async fn last_created(&self) -> Result<Option<IssueAggregate>, RepositoryError> {
        todo!()
    }

    async fn save(&self, event: IssueAggregateEvent) -> Result<(), RepositoryError> {
        todo!()
    }
}
