use std::str::FromStr;

use sqlx::{any::AnyRow, FromRow, Row};

use crate::migration_status::{MigrationStatus, Value, Version};

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

impl<'r> FromRow<'r, AnyRow> for MigrationStatusRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            current_version: row.try_get("current_version")?,
            updated_version: row.try_get("updated_version")?,
            value: row.try_get("value")?,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // TODO
    }
}
