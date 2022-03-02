use std::{fs, path::PathBuf, str::FromStr};

use sqlx::{
    any::AnyConnectOptions,
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    AnyPool,
};
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
    pub async fn new(data_dir: PathBuf) -> Result<Self, IssueRepositoryError> {
        if !data_dir.exists() {
            fs::create_dir_all(data_dir.as_path()).map_err(|_| IssueRepositoryError::IO)?;
        }
        let path = data_dir.join("command.sqlite");
        let options = SqliteConnectOptions::from_str(&format!(
            "sqlite:{}?mode=rwc",
            path.to_str().ok_or(IssueRepositoryError::IO)?
        ))
        .map_err(|_| IssueRepositoryError::IO)?
        .journal_mode(SqliteJournalMode::Delete);
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
