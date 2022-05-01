mod migration_status_value;
mod version;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use sqlx::{
        any::{AnyArguments, AnyRow},
        query::Query,
        Any, AnyPool, FromRow, Row,
    };

    use crate::{migration_status_value::MigrationStatusValue, version::Version};

    trait Migration {
        fn migrate(&self);
        fn version(&self) -> u32;
    }

    #[derive(Debug, thiserror::Error)]
    enum Error {
        #[error("already applied")]
        AlreadyApplied,
        #[error("already in progress")]
        AlreadyInProgress,
        #[error("not in progress")]
        NotInProgress,
    }

    #[derive(Debug)]
    struct MigrationStatus {
        current_version: Version,
        updated_version: Option<Version>,
        value: MigrationStatusValue,
    }

    impl MigrationStatus {
        fn complete(&self) -> Result<MigrationStatus, Error> {
            if self.value != MigrationStatusValue::InProgress {
                return Err(Error::NotInProgress);
            }
            Ok(Self {
                current_version: self.updated_version.unwrap(), // FIXME
                updated_version: None,
                value: MigrationStatusValue::Completed,
            })
        }

        fn in_progress(&self, version: Version) -> Result<MigrationStatus, Error> {
            if self.value != MigrationStatusValue::Completed {
                return Err(Error::AlreadyInProgress);
            }
            if self.current_version >= version {
                return Err(Error::AlreadyApplied);
            }

            Ok(Self {
                current_version: self.current_version,
                updated_version: Some(version),
                value: MigrationStatusValue::InProgress,
            })
        }
    }

    struct MigrationStatusRow {
        current_version: i64,
        updated_version: Option<i64>,
        value: String,
    }

    impl MigrationStatusRow {
        fn current_version(&self) -> Version {
            Version::try_from(self.current_version).expect("persisted current_version is invalid")
        }

        fn updated_version(&self) -> Option<Version> {
            self.updated_version
                .map(Version::try_from)
                .transpose()
                .expect("persisted updated_version is invalid")
        }

        fn value(&self) -> MigrationStatusValue {
            MigrationStatusValue::from_str(self.value.as_str())
                .expect("persisted migration_status is invalid")
        }
    }

    impl From<MigrationStatusRow> for MigrationStatus {
        fn from(row: MigrationStatusRow) -> Self {
            Self {
                current_version: row.current_version(),
                updated_version: row.updated_version(),
                value: row.value(),
            }
        }
    }

    impl<'r> FromRow<'r, AnyRow> for MigrationStatusRow {
        fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
            Ok(Self {
                current_version: row.try_get("current_version")?,
                updated_version: row.try_get("updated_version")?,
                value: row.try_get("value")?,
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

            sqlx::query(include_str!("./sql/create_table.sql"))
                .execute(&mut transaction)
                .await?;

            sqlx::query(include_str!("./sql/insert.sql"))
                .execute(&mut transaction)
                .await?;

            transaction.commit().await
        }

        async fn load(&self) -> sqlx::Result<MigrationStatus> {
            let mut transaction = self.pool.begin().await?;

            let row: MigrationStatusRow = sqlx::query_as(include_str!("./sql/select.sql"))
                .fetch_one(&mut transaction)
                .await?;

            transaction.rollback().await?;
            Ok(MigrationStatus::from(row))
        }

        async fn update_to_completed(&self, new_current_version: Version) -> sqlx::Result<()> {
            let mut transaction = self.pool.begin().await?;

            let query: Query<Any, AnyArguments> = sqlx::query(include_str!("./sql/update3.sql"))
                .bind(MigrationStatusValue::Completed.to_string())
                .bind(i64::from(new_current_version))
                .bind(MigrationStatusValue::InProgress.to_string());
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

            let query: Query<Any, AnyArguments> = sqlx::query(include_str!("./sql/update4.sql"))
                .bind(i64::from(new_current_version))
                .bind(MigrationStatusValue::InProgress.to_string())
                .bind(i64::from(old_current_version))
                .bind(MigrationStatusValue::Completed.to_string());
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
            let migration_status = migrator.load().await?;
            if migration_status.current_version >= migration_version {
                continue;
            }

            let in_progress = migration_status.in_progress(migration_version)?;

            migrator
                .update_to_in_progress(
                    migration_status.current_version,
                    in_progress.updated_version.unwrap(),
                )
                .await?;

            migration.migrate();

            // ここで失敗した場合は migration_status = in_progress で残る
            // Migration::migrate での失敗と区別がつかないため、ユーザーに手動で直してもらう
            let completed = in_progress.complete()?;
            migrator
                .update_to_completed(completed.current_version)
                .await?;
        }

        Ok(())
    }
}
