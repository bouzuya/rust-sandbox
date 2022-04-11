use std::str::FromStr;

use sqlx::{any::AnyRow, FromRow, Row};

use super::EventStreamId;

#[derive(Debug)]
pub(super) struct EventStreamRow {
    id: String,
    version: i64,
}

impl<'r> FromRow<'r, AnyRow> for EventStreamRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            version: row.get("version"),
        })
    }
}
impl EventStreamRow {
    pub(super) fn id(&self) -> EventStreamId {
        EventStreamId::from_str(self.id.as_str()).expect("event_streams.id is not well-formed")
    }
}
