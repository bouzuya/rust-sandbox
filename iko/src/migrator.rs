use std::{future::Future, pin::Pin};

use sqlx::AnyPool;

use crate::{migration_status::Version, query};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("migration status error: {0}")]
    MigrationStatus(#[from] crate::migration_status::Error),
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("no rows to update")]
    Query(#[from] query::Error),
}

type Migrate = Box<dyn Fn(AnyPool) -> Pin<Box<dyn Future<Output = sqlx::Result<()>>>>>;

pub struct Migrator {
    pool: AnyPool,
    migrations: Vec<(u32, Migrate)>,
}

impl Migrator {
    pub fn new(uri: &str) -> sqlx::Result<Self> {
        Ok(Self {
            pool: AnyPool::connect_lazy(uri)?,
            migrations: vec![],
        })
    }

    pub fn add_migration<Fut>(&mut self, version: u32, migrate: impl Fn(AnyPool) -> Fut + 'static)
    where
        Fut: Future<Output = sqlx::Result<()>> + 'static,
    {
        self.migrations.push((
            version,
            Box::new(move |pool: AnyPool| Box::pin(migrate(pool))),
        ));
    }

    pub async fn migrate(&self) -> Result<(), Error> {
        self.create_table().await?;
        for (version, migrate) in self.migrations.iter() {
            let migration_version = Version::from(*version);

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

            migrate(self.pool.clone()).await?;

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
