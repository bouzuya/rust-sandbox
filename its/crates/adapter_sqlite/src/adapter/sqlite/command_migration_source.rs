use std::{borrow::Cow, future::Future, pin::Pin};

use sqlx::{
    error::BoxDynError,
    migrate::{Migration, MigrationSource, MigrationType},
};

#[derive(Debug, Default)]
pub struct CommandMigrationSource {}

impl MigrationSource<'static> for CommandMigrationSource {
    fn resolve(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Migration>, BoxDynError>> + Send + 'static>> {
        Box::pin(async move {
            let migrations = vec![
                Migration::new(
                    20210223000001,
                    Cow::from("create aggregates"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20220224000001_create_aggregates.sql"
                    )),
                ),
                Migration::new(
                    20210223000002,
                    Cow::from("create events"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20220224000002_create_events.sql"
                    )),
                ),
                Migration::new(
                    20210223000003,
                    Cow::from("create issue_ids"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20220224000003_create_issue_ids.sql"
                    )),
                ),
                Migration::new(
                  20210225000000,
                    Cow::from("alter issue_ids add issue_number"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20220225000000_alter_issue_ids_add_issue_number.sql"
                    )),
                )
            ];
            Ok(migrations)
        })
    }
}
