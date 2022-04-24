use std::str::FromStr;

use sqlx::{any::AnyConnectOptions, migrate::Migrator, AnyPool};
use use_case::{IssueBlockLinkRepositoryError, IssueRepositoryError};

use crate::{SqliteIssueBlockLinkRepository, SqliteIssueRepository};

use super::command_migration_source::CommandMigrationSource;

#[derive(Clone, Debug)]
pub struct RdbConnectionPool(AnyPool);

impl From<RdbConnectionPool> for AnyPool {
    fn from(rdb_connection_pool: RdbConnectionPool) -> Self {
        rdb_connection_pool.0
    }
}

impl RdbConnectionPool {
    // TODO: Error
    pub async fn new(connection_uri: &str) -> Result<Self, IssueRepositoryError> {
        let options =
            AnyConnectOptions::from_str(connection_uri).map_err(|_| IssueRepositoryError::IO)?;
        let pool = AnyPool::connect_with(options)
            .await
            .map_err(|_| IssueRepositoryError::IO)?;

        let migrator = Migrator::new(CommandMigrationSource::default())
            .await
            .map_err(|_| IssueRepositoryError::IO)?;
        migrator
            .run(&pool)
            .await
            .map_err(|_| IssueRepositoryError::IO)?;

        Ok(Self(pool))
    }

    pub fn issue_block_link_repository(
        &self,
    ) -> Result<SqliteIssueBlockLinkRepository, IssueBlockLinkRepositoryError> {
        SqliteIssueBlockLinkRepository::new(self.clone())
    }

    pub fn issue_repository(&self) -> Result<SqliteIssueRepository, IssueRepositoryError> {
        SqliteIssueRepository::new(self.clone())
    }
}
