use iko::{Migrations, Migrator};
use sqlx::AnyPool;

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    async fn migrate1(pool: AnyPool) -> sqlx::Result<()> {
        println!("migrate1");

        let mut transaction = pool.begin().await?;

        sqlx::query("CREATE TABLE tbl1 (col1 INTEGER PRIMARY KEY)")
            .execute(&mut transaction)
            .await?;

        transaction.commit().await
    }

    async fn migrate2(pool: AnyPool) -> sqlx::Result<()> {
        println!("migrate2");

        let mut transaction = pool.begin().await?;

        sqlx::query("INSERT INTO tbl1 (col1) VALUES (123)")
            .execute(&mut transaction)
            .await?;

        transaction.commit().await
    }

    let mut migrations = Migrations::default();
    migrations.push(1, migrate1)?;
    migrations.push(2, migrate2)?;
    let migrator = Migrator::new("sqlite::memory:")?;
    migrator.migrate(&migrations).await?;

    Ok(())
}

#[tokio::test]
async fn test2() -> anyhow::Result<()> {
    async fn migrate1(pool: AnyPool) -> sqlx::Result<()> {
        println!("migrate1");

        let mut transaction = pool.begin().await?;

        sqlx::query("CREATE TABLE tbl1 (col1 INTEGER PRIMARY KEY)")
            .execute(&mut transaction)
            .await?;

        transaction.commit().await
    }

    async fn migrate2(pool: AnyPool) -> sqlx::Result<()> {
        println!("migrate2");

        let mut transaction = pool.begin().await?;

        sqlx::query("INSERT INTO tbl1 (col1) VALUES (123)")
            .execute(&mut transaction)
            .await?;

        transaction.commit().await
    }

    async fn migrate3(pool: AnyPool) -> sqlx::Result<()> {
        println!("migrate3");

        let mut transaction = pool.begin().await?;

        sqlx::query("INSERT INTO tbl1 (col1) VALUES (456)")
            .execute(&mut transaction)
            .await?;

        transaction.commit().await
    }

    let mut migrations = Migrations::default();
    migrations.push(1, migrate1)?;
    migrations.push(2, migrate2)?;
    let migrator = Migrator::new("sqlite::memory:")?;
    migrator.migrate(&migrations).await?;

    migrations.push(3, migrate3)?;
    migrator.migrate(&migrations).await?;

    Ok(())
}
