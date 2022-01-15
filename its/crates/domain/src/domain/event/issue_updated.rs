use limited_date_time::Instant;

use crate::{IssueDue, IssueId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueUpdated {
    pub(crate) at: Instant,
    pub(crate) issue_id: IssueId,
    pub(crate) issue_due: Option<IssueDue>,
    pub(crate) version: Version,
}

impl IssueUpdated {
    pub fn from_trusted_data(
        at: Instant,
        issue_id: IssueId,
        issue_due: Option<IssueDue>,
        version: Version,
    ) -> Self {
        Self::new(at, issue_id, issue_due, version)
    }

    pub(crate) fn new(
        at: Instant,
        issue_id: IssueId,
        issue_due: Option<IssueDue>,
        version: Version,
    ) -> Self {
        Self {
            at,
            issue_id,
            issue_due,
            version,
        }
    }

    pub fn at(&self) -> Instant {
        self.at
    }

    pub fn issue_id(&self) -> &IssueId {
        &self.issue_id
    }

    pub fn issue_due(&self) -> Option<IssueDue> {
        self.issue_due
    }

    pub fn version(&self) -> Version {
        self.version
    }
}

// TODO: tests
