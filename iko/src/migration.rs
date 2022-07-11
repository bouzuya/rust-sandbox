use std::{future::Future, pin::Pin};

use sqlx::any::AnyPool;

use crate::migration_status::Version;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
pub type MigrateArg = AnyPool;
pub type MigrateResult = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
pub type Migrate = Box<dyn Fn(MigrateArg) -> BoxFuture<'static, Result<()>>>;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);

pub struct Migration {
    migrate: Migrate,
    version: Version,
}

impl Migration {
    pub fn migrate(&self) -> &Migrate {
        &self.migrate
    }

    pub fn version(&self) -> Version {
        self.version
    }
}

impl<F, Fut> From<(u32, F)> for Migration
where
    F: Clone + 'static + Fn(AnyPool) -> Fut,
    Fut: Future<Output = MigrateResult>,
{
    fn from((version, migrate): (u32, F)) -> Self {
        Self {
            migrate: Box::new(move |pool: AnyPool| -> BoxFuture<'static, Result<()>> {
                let f = migrate.clone();
                Box::pin(async move { f(pool).await.map_err(|e| Error(e.to_string())) })
            }),
            version: Version::from(version),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::AnyPool;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        // #[derive(Debug, thiserror::Error)]
        // #[error("error")]
        // struct UserError;
        async fn migrate(
            _pool: AnyPool,
        ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            // Err(UserError)?
            Ok(())
        }

        let migration = Migration::from((1_u32, migrate));
        assert_eq!(migration.version(), Version::from(1_u32));
        let pool = AnyPool::connect_lazy("sqlite::memory:")?;
        migration.migrate()(pool).await?;

        Ok(())
    }
}
