use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use async_trait::async_trait;
use domain::{
    aggregate::{
        IssueBlockLinkAggregate, IssueBlockLinkAggregateError, IssueBlockLinkAggregateEvent,
    },
    IssueBlockLinkId,
};

use sqlx::{any::AnyRow, AnyPool, FromRow, Row};
use use_case::{IssueBlockLinkRepository, IssueBlockLinkRepositoryError};

use crate::SqliteConnectionPool;

use super::event_store::AggregateId;

#[derive(Debug)]
struct IssueBlockLinkIdRow {
    issue_block_link_id: String,
    aggregate_id: String,
}

impl IssueBlockLinkIdRow {
    fn issue_block_link_id(&self) -> IssueBlockLinkId {
        IssueBlockLinkId::from_str(self.issue_block_link_id.as_str()).unwrap()
    }

    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from_str(self.aggregate_id.as_str()).unwrap()
    }
}

impl<'r> FromRow<'r, AnyRow> for IssueBlockLinkIdRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            issue_block_link_id: row.get("issue_block_link_id"),
            aggregate_id: row.get("aggregate_id"),
        })
    }
}

#[derive(Debug)]
pub struct SqliteIssueBlockLinkRepository {
    pool: AnyPool,
}

impl SqliteIssueBlockLinkRepository {
    pub async fn new(
        connection_pool: SqliteConnectionPool,
    ) -> Result<Self, IssueBlockLinkRepositoryError> {
        Ok(Self {
            pool: AnyPool::from(connection_pool),
        })
    }
}

#[async_trait]
impl IssueBlockLinkRepository for SqliteIssueBlockLinkRepository {
    async fn find_by_id(
        &self,
        _issue_block_link_id: &IssueBlockLinkId,
    ) -> Result<Option<IssueBlockLinkAggregate>, IssueBlockLinkRepositoryError> {
        todo!()
    }

    async fn save(
        &self,
        _event: IssueBlockLinkAggregateEvent,
    ) -> Result<(), IssueBlockLinkRepositoryError> {
        todo!()
    }
}
