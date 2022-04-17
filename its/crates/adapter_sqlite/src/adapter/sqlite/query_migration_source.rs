use std::{borrow::Cow, future::Future, pin::Pin};

use sqlx::{
    error::BoxDynError,
    migrate::{Migration, MigrationSource, MigrationType},
};

#[derive(Debug, Default)]
pub struct QueryMigrationSource {}

impl MigrationSource<'static> for QueryMigrationSource {
    fn resolve(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Migration>, BoxDynError>> + Send + 'static>> {
        Box::pin(async move {
            let migrations = vec![
                Migration::new(
                    20220417000001,
                    Cow::from("create_issues"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/query/migrations/20220417000001_create_issues.sql"
                    )),
                ),
                Migration::new(
                    20220417000002,
                    Cow::from("create_issue_block_links"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/query/migrations/20220417000002_create_issue_block_links.sql"
                    )),
                ),
                Migration::new(
                    20220417000003,
                    Cow::from("alter_issues_add_resolution"),
                    MigrationType::Simple,
                    Cow::from(include_str!(
                        "../../../sql/query/migrations/20220417000003_alter_issues_add_resolution.sql"
                    )),
                ),
            ];
            Ok(migrations)
        })
    }
}
