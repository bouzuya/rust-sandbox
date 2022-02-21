use crate::{IssueId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
struct IssueBlockLinkAggregate {
    issue_id: IssueId,
    blocked_issue_id: IssueId,
    blocked: bool,
    version: Version,
}

impl IssueBlockLinkAggregate {
    pub(crate) fn block(issue_id: IssueId, blocked_issue_id: IssueId) -> Self {
        Self {
            issue_id,
            blocked_issue_id,
            blocked: true,
            version: Version::from(1_u64),
        }
    }

    pub(crate) fn unblock(&mut self) {
        // TODO: check blocked
        self.blocked = false;
    }
}
