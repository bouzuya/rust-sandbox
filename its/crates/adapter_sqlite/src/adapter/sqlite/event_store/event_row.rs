use std::str::FromStr;

use sqlx::{any::AnyRow, FromRow, Row};

use super::{event_id::EventId, Event, EventStreamId, EventStreamSeq};

#[derive(Debug)]
pub(super) struct EventRow {
    id: String,
    event_stream_id: String,
    data: String,
    version: i64,
}

impl EventRow {
    fn id(&self) -> EventId {
        EventId::from_str(self.id.as_str()).expect("events.id is not well-formed")
    }

    fn event_stream_id(&self) -> EventStreamId {
        EventStreamId::from_str(self.event_stream_id.as_str())
            .expect("events.event_stream_id is not well-formed")
    }

    fn data(&self) -> String {
        self.data.to_owned()
    }

    fn version(&self) -> EventStreamSeq {
        EventStreamSeq::try_from(self.version).expect("events.version is not well-formed")
    }
}

impl<'r> FromRow<'r, AnyRow> for EventRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            event_stream_id: row.get("event_stream_id"),
            data: row.get("data"),
            version: row.get("version"),
        })
    }
}

impl From<EventRow> for Event {
    fn from(row: EventRow) -> Self {
        Self {
            id: row.id(),
            stream_id: row.event_stream_id(),
            stream_seq: row.version(),
            data: row.data(),
        }
    }
}
