use std::{future::Future, pin::Pin};

use sqlx::AnyPool;

use crate::migration_status::Version;

type MigrateRet = Pin<Box<dyn Future<Output = Result<()>> + 'static>>;
pub type Migrate = Box<dyn Fn(AnyPool) -> MigrateRet>;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("unknown: {0}")]
    Unknown(String),
}

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
    F: Fn(AnyPool) -> Fut + 'static,
    Fut: Future<
        Output = std::result::Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>,
    >,
{
    fn from((version, migrate): (u32, F)) -> Self {
        Self {
            migrate: Box::new(move |pool: AnyPool| -> MigrateRet {
                Box::pin(async {
                    migrate(pool)
                        .await
                        .map_err(|e| Error::Unknown(e.to_string()))
                })
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
