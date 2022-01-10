use limited_date_time::Instant;

use crate::{IssueId, IssueTitle, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCreated {
    pub at: Instant,
    pub issue_id: IssueId,
    pub issue_title: IssueTitle,
    pub version: Version,
}
