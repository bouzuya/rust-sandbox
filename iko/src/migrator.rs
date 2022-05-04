use async_trait::async_trait;
use sqlx::AnyPool;

use crate::{migration_status::Version, query};

#[async_trait]
pub trait Migration {
    async fn migrate(&self, pool: AnyPool) -> sqlx::Result<()>;
    fn version(&self) -> u32;
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
    migrations: Vec<Box<dyn Migration>>,
}

impl Migrator {
    pub fn new(uri: &str) -> sqlx::Result<Self> {
        Ok(Self {
            pool: AnyPool::connect_lazy(uri)?,
            migrations: vec![],
        })
    }

    pub fn add_migration(&mut self, migration: impl Migration + 'static) {
        self.migrations.push(Box::new(migration));
    }

    pub async fn create_table(&self) -> sqlx::Result<()> {
        let mut transaction = self.pool.begin().await?;
        query::create_migration_status_table(&mut transaction).await?;
        query::insert_migration_status(&mut transaction).await?;
        transaction.commit().await
    }

    pub async fn migrate(&self) -> Result<(), Error> {
        for migration in self.migrations.iter() {
            let migration_version = Version::from(migration.version());

            let mut transaction = self.pool.begin().await?;
            let migration_status = query::select_migration_status(&mut transaction).await?;
            transaction.commit().await?;

            if migration_status.current_version() >= migration_version {
                continue;
            }

            let mut transaction = self.pool.begin().await?;
            let in_progress = migration_status.in_progress(migration_version)?;
            query::update_migration_status(&mut transaction, &migration_status, &in_progress)
                .await?;
            transaction.commit().await?;

            migration.migrate(self.pool.clone()).await?;

            // ここで失敗した場合は migration_status = in_progress で残る
            // Migration::migrate での失敗と区別がつかないため、ユーザーに手動で直してもらう
            let mut transaction = self.pool.begin().await?;
            let completed = in_progress.complete()?;
            query::update_migration_status(&mut transaction, &in_progress, &completed).await?;
            transaction.commit().await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // TODO
    }
}
