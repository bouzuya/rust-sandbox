use limited_date_time::Instant;

use crate::{IssueId, IssueTitle, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCreated {
    pub(crate) at: Instant,
    pub(crate) issue_id: IssueId,
    pub(crate) issue_title: IssueTitle,
    pub(crate) version: Version,
}

impl IssueCreated {
    pub fn from_trusted_data(
        at: Instant,
        issue_id: IssueId,
        issue_title: IssueTitle,
        version: Version,
    ) -> Self {
        Self::new(at, issue_id, issue_title, version)
    }

    pub(crate) fn new(
        at: Instant,
        issue_id: IssueId,
        issue_title: IssueTitle,
        version: Version,
    ) -> Self {
        Self {
            at,
            issue_id,
            issue_title,
            version,
        }
    }

    pub fn at(&self) -> Instant {
        self.at
    }

    pub fn issue_id(&self) -> &IssueId {
        &self.issue_id
    }

    pub fn issue_title(&self) -> &IssueTitle {
        &self.issue_title
    }

    pub fn version(&self) -> Version {
        self.version
    }
}

// TODO: tests
