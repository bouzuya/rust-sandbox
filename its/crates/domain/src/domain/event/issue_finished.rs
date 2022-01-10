use limited_date_time::Instant;

use crate::{IssueId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueFinished {
    pub at: Instant,
    pub issue_id: IssueId,
    pub version: Version,
}
