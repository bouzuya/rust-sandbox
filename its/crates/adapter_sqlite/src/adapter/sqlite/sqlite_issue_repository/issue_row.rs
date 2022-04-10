use std::str::FromStr;

use domain::IssueId;
use sqlx::{any::AnyRow, FromRow, Row};

use crate::adapter::sqlite::event_store::AggregateId;

#[derive(Debug)]
pub(super) struct IssueIdRow {
    issue_number: i64,
    aggregate_id: String,
}

impl IssueIdRow {
    pub(super) fn issue_id(&self) -> IssueId {
        IssueId::from_str(self.issue_number.to_string().as_str())
            .expect("stored issue_number is not well-formed")
    }

    pub(super) fn aggregate_id(&self) -> AggregateId {
        AggregateId::from_str(self.aggregate_id.as_str())
            .expect("stored aggregate_id is not well-formed")
    }
}

impl<'r> FromRow<'r, AnyRow> for IssueIdRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            issue_number: row.get("issue_number"),
            aggregate_id: row.get("event_stream_id"),
        })
    }
}
