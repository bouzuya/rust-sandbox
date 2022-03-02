use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use async_trait::async_trait;
use domain::{
    aggregate::{IssueBlockLinkAggregate, IssueBlockLinkAggregateEvent},
    IssueBlockLinkId,
};

use sqlx::AnyPool;
use use_case::{IssueBlockLinkRepository, IssueBlockLinkRepositoryError};

use crate::SqliteConnectionPool;

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
