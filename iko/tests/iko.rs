use async_trait::async_trait;
use sqlx::AnyPool;

use iko::{Migration, Migrator};

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    struct Migration1 {}
    #[async_trait]
    impl Migration for Migration1 {
        async fn migrate(&self, pool: AnyPool) -> sqlx::Result<()> {
            println!("migrate1");

            let mut transaction = pool.begin().await?;

            sqlx::query("CREATE TABLE tbl1 (col1 INTEGER PRIMARY KEY)")
                .execute(&mut transaction)
                .await?;

            transaction.commit().await
        }

        fn version(&self) -> u32 {
            1
        }
    }

    struct Migration2 {}
    #[async_trait]
    impl Migration for Migration2 {
        async fn migrate(&self, pool: AnyPool) -> sqlx::Result<()> {
            println!("migrate2");

            let mut transaction = pool.begin().await?;

            sqlx::query("INSERT INTO tbl1 (col1) VALUES (123)")
                .execute(&mut transaction)
                .await?;

            transaction.commit().await
        }

        fn version(&self) -> u32 {
            2
        }
    }

    let mut migrator = Migrator::new("sqlite::memory:")?;
    migrator.create_table().await?;
    migrator.add_migration(Migration1 {});
    migrator.add_migration(Migration2 {});
    migrator.migrate().await?;

    Ok(())
}
