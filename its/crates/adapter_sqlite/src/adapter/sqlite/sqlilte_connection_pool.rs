use std::str::FromStr;

use sqlx::{any::AnyConnectOptions, migrate::Migrator, AnyPool};
use use_case::IssueRepositoryError;

use super::command_migration_source::CommandMigrationSource;

#[derive(Clone, Debug)]
pub struct SqliteConnectionPool(AnyPool);

impl From<SqliteConnectionPool> for AnyPool {
    fn from(sqlite_connection_pool: SqliteConnectionPool) -> Self {
        sqlite_connection_pool.0
    }
}

impl SqliteConnectionPool {
    pub async fn new(connection_uri: &str) -> Result<Self, IssueRepositoryError> {
        let options =
            AnyConnectOptions::from_str(connection_uri).map_err(|_| IssueRepositoryError::IO)?;
        let options = AnyConnectOptions::from(options);
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
}
