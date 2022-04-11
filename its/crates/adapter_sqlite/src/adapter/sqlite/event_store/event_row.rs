use std::str::FromStr;

use sqlx::{any::AnyRow, FromRow, Row};

use super::{AggregateVersion, Event, EventStreamId};

#[derive(Debug)]
pub(super) struct EventRow {
    event_stream_id: String,
    data: String,
    version: i64,
}

impl EventRow {
    fn event_stream_id(&self) -> EventStreamId {
        EventStreamId::from_str(self.event_stream_id.as_str())
            .expect("stored event_stream_id is not well-formed")
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
            event_stream_id: row.get("event_stream_id"),
            data: row.get("data"),
            version: row.get("version"),
        })
    }
}

impl From<EventRow> for Event {
    fn from(row: EventRow) -> Self {
        Self {
            event_stream_id: row.event_stream_id(),
            data: row.data(),
            version: row.version(),
        }
    }
}
