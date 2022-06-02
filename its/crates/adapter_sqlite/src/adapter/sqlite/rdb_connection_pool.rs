use std::str::FromStr;

use sqlx::{any::AnyConnectOptions, migrate::Migrator, AnyPool};
use use_case::{IssueBlockLinkRepositoryError, IssueRepositoryError};

use crate::{SqliteIssueBlockLinkRepository, SqliteIssueRepository};

use super::command_migration_source::CommandMigrationSource;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("issue repository error: {0}")]
    IssueRepository(#[from] IssueRepositoryError),
    #[error("issue block link repository error: {0}")]
    IssueBlockLinkRepository(#[from] IssueBlockLinkRepositoryError),
    #[error("migrate error: {0}")]
    Migrate(#[from] iko::MigratorError),
    #[error("migrations error: {0}")]
    Migrations(#[from] iko::MigrationsError),
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

        async fn migrate1(
            pool: AnyPool,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            let migrator = Migrator::new(CommandMigrationSource::default()).await?;
            migrator.run(&pool).await?;
            Ok(())
        }

        let iko_migrator = iko::Migrator::new(pool.clone());
        let mut iko_migrations = iko::Migrations::default();
        iko_migrations.push(1, migrate1)?;
        iko_migrator.migrate(&iko_migrations).await?;

        Ok(Self(pool))
    }

    pub fn issue_block_link_repository(&self) -> Result<SqliteIssueBlockLinkRepository> {
        Ok(SqliteIssueBlockLinkRepository::new(self.clone())?)
    }

    pub fn issue_repository(&self) -> Result<SqliteIssueRepository> {
        Ok(SqliteIssueRepository::new(self.clone())?)
    }
}
