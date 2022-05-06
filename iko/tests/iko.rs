use iko::Migrator;
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

    let mut migrator = Migrator::new("sqlite::memory:")?;
    migrator.add_migration(1, migrate1);
    migrator.add_migration(2, migrate2);
    migrator.migrate().await?;

    Ok(())
}
