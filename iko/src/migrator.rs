use sqlx::AnyPool;

use crate::{migrations::Migrations, query};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("migration status error: {0}")]
    MigrationStatus(#[from] crate::migration_status::Error),
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("no rows to update: {0}")]
    Query(#[from] query::Error),
    #[error("version 0 is reserved")]
    ReservedVersion,
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

    pub async fn migrate(&self, migrations: &Migrations) -> Result<(), Error> {
        self.create_table().await?;
        for migration in migrations.iter() {
            let mut transaction = self.pool.begin().await?;
            let migration_status = query::select_migration_status(&mut transaction).await?;
            transaction.commit().await?;

            if migration_status.current_version() >= migration.version() {
                continue;
            }

            let mut transaction = self.pool.begin().await?;
            let in_progress = migration_status.in_progress(migration.version())?;
            query::update_migration_status(&mut transaction, &migration_status, &in_progress)
                .await?;
            transaction.commit().await?;

            migration.migrate()(self.pool.clone()).await?;

            // ここで失敗した場合は migration_status = in_progress で残る
            // Migration::migrate での失敗と区別がつかないため、ユーザーに手動で直してもらう
            let mut transaction = self.pool.begin().await?;
            let completed = in_progress.complete()?;
            query::update_migration_status(&mut transaction, &in_progress, &completed).await?;
            transaction.commit().await?;
        }
        Ok(())
    }

    async fn create_table(&self) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;
        query::create_migration_status_table(&mut transaction).await?;
        query::insert_migration_status(&mut transaction).await?;
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
