#[cfg(test)]
mod tests {
    use sqlx::{
        any::{AnyArguments, AnyRow},
        query::Query,
        Any, AnyPool, FromRow, Row, Transaction,
    };

    trait Migration {
        fn migrate(&self);
        fn version(&self) -> u32;
    }
    struct Migrator {
        pool: AnyPool,
    }
    impl Migrator {
        fn new(uri: &str) -> sqlx::Result<Self> {
            Ok(Self {
                pool: AnyPool::connect_lazy(uri)?,
            })
        }

        async fn create_table(&self) -> sqlx::Result<()> {
            let mut transaction = self.pool.begin().await?;

            sqlx::query("CREATE TABLE database_version (current_version INTEGER PRIMARY KEY)")
                .execute(&mut transaction)
                .await?;

            sqlx::query("INSERT INTO database_version(current_version) VALUES (0)")
                .execute(&mut transaction)
                .await?;

            transaction.commit().await
        }

        async fn begin(&self) -> sqlx::Result<Transaction<'_, Any>> {
            self.pool.begin().await
        }

        async fn load_current_version(&self) -> sqlx::Result<u32> {
            let mut transaction = self.pool.begin().await?;

            struct DatabaseVersionRow {
                current_version: u32,
            }
            impl<'r> FromRow<'r, AnyRow> for DatabaseVersionRow {
                fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
                    let current_version: i64 = row.get("current_version");
                    Ok(Self {
                        current_version: u32::try_from(current_version).unwrap(),
                    })
                }
            }
            let row: DatabaseVersionRow =
                sqlx::query_as("SELECT current_version FROM database_version")
                    .fetch_one(&mut transaction)
                    .await?;

            transaction.rollback().await?;
            Ok(row.current_version)
        }

        async fn save_current_version(
            &self,
            transaction: &mut Transaction<'_, Any>,
            old_current_version: u32,
            new_current_version: u32,
        ) -> sqlx::Result<()> {
            let query: Query<Any, AnyArguments> = sqlx::query(
                "UPDATE database_version SET current_version = $1 WHERE current_version = $2",
            )
            .bind(i64::from(new_current_version))
            .bind(i64::from(old_current_version));
            let rows_affected = query.execute(transaction).await?.rows_affected();
            if rows_affected != 1 {
                todo!();
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        struct Migration1 {}
        impl Migration for Migration1 {
            fn migrate(&self) {
                println!("migrate1");
            }

            fn version(&self) -> u32 {
                1
            }
        }

        struct Migration2 {}
        impl Migration for Migration2 {
            fn migrate(&self) {
                println!("migrate2");
            }

            fn version(&self) -> u32 {
                2
            }
        }

        let migrator = Migrator::new("sqlite::memory:")?;
        migrator.create_table().await?;

        let migrations: Vec<Box<dyn Migration>> =
            vec![Box::new(Migration1 {}), Box::new(Migration2 {})];
        for migration in migrations {
            let current_version = migrator.load_current_version().await?;
            if current_version >= migration.version() {
                continue;
            }
            let mut transaction = migrator.begin().await?;

            migrator
                .save_current_version(&mut transaction, current_version, migration.version())
                .await?;

            migration.migrate();

            transaction.commit().await?;
        }

        Ok(())
    }
}
