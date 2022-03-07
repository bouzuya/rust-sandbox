use std::str::FromStr;

use domain::IssueBlockLinkId;

use sqlx::{any::AnyRow, FromRow, Row};

use super::event_store::AggregateId;

#[derive(Debug)]
pub(super) struct IssueBlockLinkIdRow {
    issue_block_link_id: String,
    aggregate_id: String,
}

impl IssueBlockLinkIdRow {
    pub(super) fn issue_block_link_id(&self) -> IssueBlockLinkId {
        IssueBlockLinkId::from_str(&self.issue_block_link_id)
            .expect("stored issue_block_link_id is not well-formed")
    }

    pub(super) fn aggregate_id(&self) -> AggregateId {
        AggregateId::from_str(&self.aggregate_id)
            .expect("stored issue_block_link_id is not well-formed")
    }
}

impl<'r> FromRow<'r, AnyRow> for IssueBlockLinkIdRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            issue_block_link_id: row.get("issue_block_link_id"),
            aggregate_id: row.get("aggregate_id"),
        })
    }
}
