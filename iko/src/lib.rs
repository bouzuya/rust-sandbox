mod version;

#[cfg(test)]
mod tests {
    use std::{fmt::Display, str::FromStr};

    use sqlx::{
        any::{AnyArguments, AnyRow},
        query::Query,
        Any, AnyPool, FromRow, Row,
    };

    use crate::version::Version;

    trait Migration {
        fn migrate(&self);
        fn version(&self) -> u32;
    }

    enum MigrationStatus {
        InProgress,
        Completed,
    }

    impl FromStr for MigrationStatus {
        type Err = sqlx::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "in_progress" => MigrationStatus::InProgress,
                "completed" => MigrationStatus::Completed,
                _ => todo!(),
            })
        }
    }

    impl Display for MigrationStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    MigrationStatus::InProgress => "in_progress",
                    MigrationStatus::Completed => "completed",
                }
            )
        }
    }

    struct DatabaseVersion {
        current_version: Version,
        migration_status: MigrationStatus,
    }

    struct DatabaseVersionRow {
        current_version: i64,
        migration_status: String,
    }

    impl DatabaseVersionRow {
        fn current_version(&self) -> Version {
            Version::try_from(self.current_version).expect("persisted current_version is invalid")
        }

        fn migration_status(&self) -> MigrationStatus {
            MigrationStatus::from_str(self.migration_status.as_str())
                .expect("persisted migration_status is invalid")
        }
    }

    impl From<DatabaseVersionRow> for DatabaseVersion {
        fn from(row: DatabaseVersionRow) -> Self {
            Self {
                current_version: row.current_version(),
                migration_status: row.migration_status(),
            }
        }
    }

    impl<'r> FromRow<'r, AnyRow> for DatabaseVersionRow {
        fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
            Ok(Self {
                current_version: row.try_get("current_version")?,
                migration_status: row.try_get("migration_status")?,
            })
        }
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

            sqlx::query("CREATE TABLE database_version (current_version INTEGER PRIMARY KEY, migration_status VARCHAR(11) NOT NULL)")
                .execute(&mut transaction)
                .await?;

            sqlx::query("INSERT INTO database_version(current_version, migration_status) VALUES (0, 'completed')")
                .execute(&mut transaction)
                .await?;

            transaction.commit().await
        }

        async fn load(&self) -> sqlx::Result<DatabaseVersion> {
            let mut transaction = self.pool.begin().await?;

            let row: DatabaseVersionRow =
                sqlx::query_as("SELECT current_version, migration_status FROM database_version")
                    .fetch_one(&mut transaction)
                    .await?;

            transaction.rollback().await?;
            Ok(DatabaseVersion::from(row))
        }

        async fn update_to_completed(&self, new_current_version: Version) -> sqlx::Result<()> {
            let mut transaction = self.pool.begin().await?;

            let query: Query<Any, AnyArguments> = sqlx::query(
                "UPDATE database_version SET migration_status = $1 WHERE current_version = $2 AND migration_status = $3",
            )
            .bind(MigrationStatus::Completed.to_string())
            .bind(i64::from(new_current_version))
            .bind(MigrationStatus::InProgress.to_string());
            let rows_affected = query.execute(&mut transaction).await?.rows_affected();
            if rows_affected != 1 {
                todo!();
            }

            transaction.commit().await
        }

        async fn update_to_in_progress(
            &self,
            old_current_version: Version,
            new_current_version: Version,
        ) -> sqlx::Result<()> {
            let mut transaction = self.pool.begin().await?;

            let query: Query<Any, AnyArguments> = sqlx::query(
                "UPDATE database_version SET current_version = $1, migration_status = $2 WHERE current_version = $3 AND migration_status = $4",
            )
            .bind(i64::from(new_current_version))
            .bind(MigrationStatus::InProgress.to_string())
            .bind(i64::from(old_current_version))
            .bind(MigrationStatus::Completed.to_string());
            let rows_affected = query.execute(&mut transaction).await?.rows_affected();
            if rows_affected != 1 {
                todo!();
            }

            transaction.commit().await
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
            let migration_version = Version::from(migration.version());
            let database_version = migrator.load().await?;
            if database_version.current_version >= migration_version {
                continue;
            }

            migrator
                .update_to_in_progress(database_version.current_version, migration_version)
                .await?;

            migration.migrate();

            // ここで失敗した場合は migration_status = in_progress で残る
            // Migration::migrate での失敗と区別がつかないため、ユーザーに手動で直してもらう
            migrator.update_to_completed(migration_version).await?;
        }

        Ok(())
    }
}
