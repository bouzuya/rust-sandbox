mod migrate1;
mod migrate2;

use migrate1::*;
use migrate2::*;
use sqlx::AnyPool;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("migration {0}")]
    Migration(#[from] iko::MigrationsError),
    #[error("migrator {0}")]
    Migrator(#[from] iko::MigratorError),
}

pub async fn migrate(pool: AnyPool) -> Result<()> {
    let iko_migrator = iko::Migrator::new(pool.clone());
    let mut iko_migrations = iko::Migrations::default();
    iko_migrations.push(1, migrate1)?;
    iko_migrations.push(2, migrate2)?;
    iko_migrator.migrate(&iko_migrations).await?;
    Ok(())
}
