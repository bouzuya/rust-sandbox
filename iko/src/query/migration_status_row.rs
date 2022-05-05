use std::str::FromStr;

use sqlx::FromRow;

use crate::migration_status::{MigrationStatus, Value, Version};

#[derive(Debug, FromRow)]
pub struct MigrationStatusRow {
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

    fn value(&self) -> Value {
        Value::from_str(self.value.as_str()).expect("persisted migration_status is invalid")
    }
}

impl From<MigrationStatusRow> for MigrationStatus {
    fn from(row: MigrationStatusRow) -> Self {
        match row.value() {
            Value::InProgress => MigrationStatus::InProgress {
                current_version: row.current_version(),
                updated_version: row
                    .updated_version()
                    .expect("persisted updated_version is invalid"),
            },
            Value::Completed => MigrationStatus::Completed {
                current_version: row.current_version(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::migration_status::{MigrationStatus, Version};

    use super::MigrationStatusRow;

    #[test]
    fn migration_status_conversion_test() {
        let initial = MigrationStatusRow {
            current_version: 0,
            updated_version: None,
            value: "completed".to_string(),
        };
        assert_eq!(
            MigrationStatus::from(initial),
            MigrationStatus::Completed {
                current_version: Version::from(0),
            }
        );

        let in_progress = MigrationStatusRow {
            current_version: 0,
            updated_version: Some(1),
            value: "in_progress".to_string(),
        };
        assert_eq!(
            MigrationStatus::from(in_progress),
            MigrationStatus::InProgress {
                current_version: Version::from(0),
                updated_version: Version::from(1),
            }
        );
    }
}
