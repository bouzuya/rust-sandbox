use std::str::FromStr;

use sqlx::{any::AnyRow, FromRow, Row};

use super::{AggregateId, AggregateVersion, Event};

#[derive(Debug)]
pub(super) struct EventRow {
    aggregate_id: String,
    data: String,
    version: i64,
}

impl EventRow {
    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from_str(self.aggregate_id.as_str())
            .expect("stored aggregate_id is not well-formed")
    }

    fn data(&self) -> String {
        self.data.to_owned()
    }

    fn version(&self) -> AggregateVersion {
        AggregateVersion::try_from(self.version).expect("stored version is not well-formed")
    }
}

impl<'r> FromRow<'r, AnyRow> for EventRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            aggregate_id: row.get("event_stream_id"),
            data: row.get("data"),
            version: row.get("version"),
        })
    }
}

impl From<EventRow> for Event {
    fn from(row: EventRow) -> Self {
        Self {
            aggregate_id: row.aggregate_id(),
            data: row.data(),
            version: row.version(),
        }
    }
}
