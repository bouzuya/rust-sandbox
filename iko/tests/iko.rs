use iko::{MigrateArg, MigrateResult, Migrations, MigrationsError, Migrator, MigratorError};
use sqlx::AnyPool;

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    async fn migrate1(pool: MigrateArg) -> MigrateResult {
        println!("migrate1");

        let mut transaction = pool.begin().await?;

        sqlx::query("CREATE TABLE tbl1 (col1 INTEGER PRIMARY KEY)")
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    async fn migrate2(pool: MigrateArg) -> MigrateResult {
        println!("migrate2");

        let mut transaction = pool.begin().await?;

        sqlx::query("INSERT INTO tbl1 (col1) VALUES (123)")
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    let pool = AnyPool::connect("sqlite::memory:").await?;
    let mut migrations = Migrations::default();
    migrations.push(1, migrate1)?;
    migrations.push(2, migrate2)?;
    let migrator = Migrator::new(pool);
    migrator.migrate(&migrations).await?;

    Ok(())
}

#[tokio::test]
async fn test2() -> anyhow::Result<()> {
    async fn migrate1(pool: MigrateArg) -> MigrateResult {
        println!("migrate1");

        let mut transaction = pool.begin().await?;

        sqlx::query("CREATE TABLE tbl1 (col1 INTEGER PRIMARY KEY)")
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    async fn migrate2(pool: MigrateArg) -> MigrateResult {
        println!("migrate2");

        let mut transaction = pool.begin().await?;

        sqlx::query("INSERT INTO tbl1 (col1) VALUES (123)")
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    async fn migrate3(pool: MigrateArg) -> MigrateResult {
        println!("migrate3");

        let mut transaction = pool.begin().await?;

        sqlx::query("INSERT INTO tbl1 (col1) VALUES (456)")
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    let pool = AnyPool::connect("sqlite::memory:").await?;
    let mut migrations = Migrations::default();
    migrations.push(1, migrate1)?;
    migrations.push(2, migrate2)?;
    let migrator = Migrator::new(pool);
    migrator.migrate(&migrations).await?;

    migrations.push(3, migrate3)?;
    migrator.migrate(&migrations).await?;

    Ok(())
}

#[tokio::test]
async fn export_test() -> anyhow::Result<()> {
    async fn migrate1(_pool: MigrateArg) -> MigrateResult {
        Ok(())
    }

    let pool = AnyPool::connect("sqlite::memory:").await?;
    let mut migrations = Migrations::default();
    let result: Result<(), MigrationsError> = migrations.push(1, migrate1);
    result?;
    let migrator = Migrator::new(pool);
    let result: Result<(), MigratorError> = migrator.migrate(&migrations).await;
    result?;

    Ok(())
}
