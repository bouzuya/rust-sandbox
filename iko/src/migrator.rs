use std::str::FromStr;

use async_trait::async_trait;
use sqlx::{
    any::{AnyArguments, AnyRow},
    query::Query,
    Any, AnyPool, FromRow, Row,
};

use crate::{
    migration_status::MigrationStatus, migration_status_value::MigrationStatusValue,
    version::Version,
};

#[async_trait]
pub trait Migration {
    async fn migrate(&self, pool: AnyPool) -> sqlx::Result<()>;
    fn version(&self) -> u32;
}

struct MigrationStatusRow {
    current_version: i64,
    updated_version: Option<i64>,
    value: String,
}

impl MigrationStatusRow {
    fn current_version(&self) -> Version {
        Version::try_from(self.current_version).expect("persisted current_version is invalid")
    }

    fn updated_version(&self) -> Option<Version> {
        self.updated_version
            .map(Version::try_from)
            .transpose()
            .expect("persisted updated_version is invalid")
    }

    fn value(&self) -> MigrationStatusValue {
        MigrationStatusValue::from_str(self.value.as_str())
            .expect("persisted migration_status is invalid")
    }
}

impl From<MigrationStatusRow> for MigrationStatus {
    fn from(row: MigrationStatusRow) -> Self {
        match row.value() {
            MigrationStatusValue::InProgress => MigrationStatus::InProgress {
                current_version: row.current_version(),
                updated_version: row
                    .updated_version()
                    .expect("persisted updated_version is invalid"),
            },
            MigrationStatusValue::Completed => MigrationStatus::Completed {
                current_version: row.current_version(),
            },
        }
    }
}

impl<'r> FromRow<'r, AnyRow> for MigrationStatusRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            current_version: row.try_get("current_version")?,
            updated_version: row.try_get("updated_version")?,
            value: row.try_get("value")?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("migration status error: {0}")]
    MigrationStatusError(#[from] crate::migration_status::Error),
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

pub struct Migrator {
    pool: AnyPool,
}

impl Migrator {
    pub fn new(uri: &str) -> sqlx::Result<Self> {
        Ok(Self {
            pool: AnyPool::connect_lazy(uri)?,
        })
    }

    pub async fn create_table(&self) -> sqlx::Result<()> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query(include_str!("./sql/create_table.sql"))
            .execute(&mut transaction)
            .await?;

        sqlx::query(include_str!("./sql/insert.sql"))
            .execute(&mut transaction)
            .await?;

        transaction.commit().await
    }

    pub async fn migrate(&self, migrations: &[Box<dyn Migration>]) -> Result<(), Error> {
        for migration in migrations {
            let migration_version = Version::from(migration.version());
            let migration_status = self.load().await?;
            if migration_status.current_version() >= migration_version {
                continue;
            }

            let in_progress = migration_status.in_progress(migration_version)?;
            self.store(&migration_status, &in_progress).await?;

            migration.migrate(self.pool.clone()).await?;

            // ここで失敗した場合は migration_status = in_progress で残る
            // Migration::migrate での失敗と区別がつかないため、ユーザーに手動で直してもらう
            let completed = in_progress.complete()?;
            self.store(&in_progress, &completed).await?;
        }
        Ok(())
    }

    async fn load(&self) -> sqlx::Result<MigrationStatus> {
        let mut transaction = self.pool.begin().await?;

        let row: MigrationStatusRow = sqlx::query_as(include_str!("./sql/select.sql"))
            .fetch_one(&mut transaction)
            .await?;

        transaction.rollback().await?;
        Ok(MigrationStatus::from(row))
    }

    async fn store(
        &self,
        current: &MigrationStatus,
        updated: &MigrationStatus,
    ) -> sqlx::Result<()> {
        let mut transaction = self.pool.begin().await?;

        let query: Query<Any, AnyArguments> = sqlx::query(include_str!("./sql/update.sql"))
            .bind(i64::from(updated.current_version()))
            .bind(updated.updated_version().map(i64::from))
            .bind(updated.value().to_string())
            .bind(i64::from(current.current_version()))
            .bind(current.value().to_string());
        let rows_affected = query.execute(&mut transaction).await?.rows_affected();
        if rows_affected != 1 {
            todo!();
        }

        transaction.commit().await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // TODO
    }
}
