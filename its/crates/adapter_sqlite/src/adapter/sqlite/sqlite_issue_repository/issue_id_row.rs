use std::str::FromStr;

use domain::IssueId;
use sqlx::{any::AnyRow, FromRow, Row};

use crate::adapter::sqlite::event_store::EventStreamId;

#[derive(Debug)]
pub(super) struct IssueIdRow {
    issue_number: i64,
    event_stream_id: String,
}

impl IssueIdRow {
    pub(super) fn issue_id(&self) -> IssueId {
        IssueId::from_str(self.issue_number.to_string().as_str())
            .expect("issue_ids.issue_number is not well-formed")
    }

    pub(super) fn event_stream_id(&self) -> EventStreamId {
        EventStreamId::from_str(self.event_stream_id.as_str())
            .expect("issue_ids.event_stream_id is not well-formed")
    }
}

impl<'r> FromRow<'r, AnyRow> for IssueIdRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            issue_number: row.get("issue_number"),
            event_stream_id: row.get("event_stream_id"),
        })
    }
}
