use std::str::FromStr;

use domain::IssueCommentId;
use event_store::EventStreamId;
use sqlx::{any::AnyRow, FromRow, Row};

#[derive(Debug)]
pub(super) struct IssueCommentIdRow {
    issue_comment_id: String,
    event_stream_id: String,
}

impl IssueCommentIdRow {
    #[allow(dead_code)]
    pub(super) fn issue_comment_id(&self) -> IssueCommentId {
        IssueCommentId::from_str(self.issue_comment_id.to_string().as_str())
            .expect("issue_comment_ids.issue_comment_id is not well-formed")
    }

    pub(super) fn event_stream_id(&self) -> EventStreamId {
        EventStreamId::from_str(self.event_stream_id.as_str())
            .expect("issue_comment_ids.event_stream_id is not well-formed")
    }
}

impl<'r> FromRow<'r, AnyRow> for IssueCommentIdRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            issue_comment_id: row.get("issue_comment_id"),
            event_stream_id: row.get("event_stream_id"),
        })
    }
}
