use std::str::FromStr;

use sqlx::{any::AnyConnectOptions, AnyPool};
use use_case::IssueBlockLinkRepositoryError;

use crate::{SqliteIssueBlockLinkRepository, SqliteIssueRepository};

use super::migration::migrate;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("issue repository error: {0}")]
    IssueRepository(#[from] super::sqlite_issue_repository::Error),
    #[error("issue block link repository error: {0}")]
    IssueBlockLinkRepository(#[from] IssueBlockLinkRepositoryError),
    #[error("migration error: {0}")]
    Migration(#[from] super::migration::Error),
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Clone, Debug)]
pub struct RdbConnectionPool(AnyPool);

impl From<RdbConnectionPool> for AnyPool {
    fn from(rdb_connection_pool: RdbConnectionPool) -> Self {
        rdb_connection_pool.0
    }
}

impl RdbConnectionPool {
    pub async fn new(connection_uri: &str) -> Result<Self> {
        let options = AnyConnectOptions::from_str(connection_uri)?;
        let pool = AnyPool::connect_with(options).await?;

        migrate(pool.clone()).await?;

        Ok(Self(pool))
    }

    pub fn issue_block_link_repository(&self) -> Result<SqliteIssueBlockLinkRepository> {
        Ok(SqliteIssueBlockLinkRepository::new(self.clone())?)
    }

    pub fn issue_repository(&self) -> Result<SqliteIssueRepository> {
        Ok(SqliteIssueRepository::new(self.clone())?)
    }
}
