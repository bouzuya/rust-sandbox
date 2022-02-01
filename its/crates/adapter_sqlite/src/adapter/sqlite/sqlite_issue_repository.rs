use std::{path::Path, str::FromStr};

use anyhow::Context;
use domain::{
    aggregate::{IssueAggregate, IssueAggregateEvent},
    IssueId,
};
use sqlx::{
    any::AnyConnectOptions,
    pool::PoolConnection,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Any, AnyPool,
};
use use_case::{IssueRepository, RepositoryError};

#[derive(Debug, Default)]
pub struct SqliteIssueRepository {}

impl IssueRepository for SqliteIssueRepository {
    fn find_by_id(&self, issue_id: &IssueId) -> Result<Option<IssueAggregate>, RepositoryError> {
        todo!()
    }

    fn last_created(&self) -> Result<Option<IssueAggregate>, RepositoryError> {
        todo!()
    }

    fn save(&self, event: IssueAggregateEvent) -> Result<(), RepositoryError> {
        todo!()
    }
}
