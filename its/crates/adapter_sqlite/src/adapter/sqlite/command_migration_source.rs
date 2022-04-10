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
                        "../../../sql/command/migrations/20210223000001_create_aggregates.sql"
                    )),
                ),
                Migration::new(
                    20210223000002,
                    Cow::from("create events"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20210223000002_create_events.sql"
                    )),
                ),
                Migration::new(
                    20210223000003,
                    Cow::from("create issue_ids"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20210223000003_create_issue_ids.sql"
                    )),
                ),
                Migration::new(
                    20210225000000,
                    Cow::from("alter issue_ids add issue_number"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20220225000000_alter_issue_ids_add_issue_number.sql"
                    )),
                ),
                Migration::new(
                    20220303000001,
                    Cow::from("create issue_block_link_ids"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20220303000001_create_issue_block_link_ids.sql"
                    )),
                ),
                Migration::new(
                    20220410000001,
                    Cow::from("create event_streams"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20220410000001_create_event_streams.sql"
                    )),
                ),
                Migration::new(
                    20220410000002,
                    Cow::from("alter events rename aggregate_id"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20220410000002_alter_events_rename_aggregate_id.sql"
                    )),
                ),
                Migration::new(
                    20220410000003,
                    Cow::from("alter issue_ids rename aggregate_id"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20220410000003_alter_issue_ids_rename_aggregate_id.sql"
                    )),
                ),
                Migration::new(
                    20220410000004,
                    Cow::from("alter issue_block_link_ids rename aggregate_id"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20220410000004_alter_issue_block_link_ids_rename_aggregate_id.sql"
                    )),
                ),
                Migration::new(
                    20220410000005,
                    Cow::from("drop aggregates"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/command/migrations/20220410000005_drop_aggregates.sql"
                    )),
                ),
            ];
            Ok(migrations)
        })
    }
}
