use std::{future::Future, pin::Pin};

use sqlx::AnyPool;

use crate::migration_status::Version;

pub type Migrate = Box<dyn Fn(AnyPool) -> Pin<Box<dyn Future<Output = Result<()>>>>>;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
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
    Fut: Future<Output = Result<()>> + 'static,
{
    fn from((version, migrate): (u32, F)) -> Self {
        Self {
            migrate: Box::new(move |pool: AnyPool| Box::pin(migrate(pool))),
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
        async fn migrate(_pool: AnyPool) -> Result<()> {
            Ok(())
        }

        let migration = Migration::from((1_u32, migrate));
        assert_eq!(migration.version(), Version::from(1_u32));
        let pool = AnyPool::connect_lazy("sqlite::memory:")?;
        migration.migrate()(pool).await?;

        Ok(())
    }
}
