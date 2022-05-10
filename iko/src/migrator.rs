use sqlx::AnyPool;

use crate::{migration, migration_status::MigrationStatus, migrations::Migrations, query};

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("migration status error: {0}")]
    MigrationStatus(#[from] crate::migration_status::Error),
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("no rows to update: {0}")]
    Query(#[from] query::Error),
    #[error("migration: {0}")]
    Migration(#[from] migration::Error),
}

pub struct Migrator {
    pool: AnyPool,
}

impl Migrator {
    pub fn new(uri: &str) -> Result<Self> {
        Ok(Self {
            pool: AnyPool::connect_lazy(uri)?,
        })
    }

    pub async fn migrate(&self, migrations: &Migrations) -> Result<()> {
        self.create_table().await?;
        for migration in migrations.iter() {
            let migration_status = self.select_migration_status().await?;
            if migration_status.current_version() >= migration.version() {
                continue;
            }

            let in_progress = migration_status.in_progress(migration.version())?;
            self.update_migration_status(&migration_status, &in_progress)
                .await?;

            migration.migrate()(self.pool.clone()).await?;

            // ここで失敗した場合は migration_status = in_progress で残る
            // Migration::migrate での失敗と区別がつかないため、ユーザーに手動で直してもらう
            let completed = in_progress.complete()?;
            self.update_migration_status(&in_progress, &completed)
                .await?;
        }
        Ok(())
    }

    async fn create_table(&self) -> Result<()> {
        let mut transaction = self.pool.begin().await?;
        query::create_migration_status_table(&mut transaction).await?;
        query::insert_migration_status(&mut transaction).await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn select_migration_status(&self) -> Result<MigrationStatus> {
        let mut transaction = self.pool.begin().await?;
        let migration_status = query::select_migration_status(&mut transaction).await?;
        transaction.commit().await?;
        Ok(migration_status)
    }

    async fn update_migration_status(
        &self,
        current: &MigrationStatus,
        updated: &MigrationStatus,
    ) -> Result<()> {
        let mut transaction = self.pool.begin().await?;
        query::update_migration_status(&mut transaction, current, updated).await?;
        transaction.commit().await?;
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
