use std::str::FromStr;

use sqlx::{any::AnyRow, FromRow, Row};

use super::AggregateId;

#[derive(Debug)]
pub(super) struct AggregateRow {
    id: String,
    version: i64,
}

impl<'r> FromRow<'r, AnyRow> for AggregateRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            version: row.get("version"),
        })
    }
}
impl AggregateRow {
    pub(super) fn id(&self) -> AggregateId {
        AggregateId::from_str(self.id.as_str()).expect("stored aggregate_id is not well-formed")
    }
}
