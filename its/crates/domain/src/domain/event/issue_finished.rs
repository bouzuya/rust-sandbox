use limited_date_time::Instant;

use crate::{IssueId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueFinished {
    pub(crate) at: Instant,
    pub(crate) issue_id: IssueId,
    pub(crate) version: Version,
}

impl IssueFinished {
    pub fn from_trusted_data(at: Instant, issue_id: IssueId, version: Version) -> Self {
        Self::new(at, issue_id, version)
    }

    pub(crate) fn new(at: Instant, issue_id: IssueId, version: Version) -> Self {
        Self {
            at,
            issue_id,
            version,
        }
    }

    pub fn at(&self) -> Instant {
        self.at
    }

    pub fn issue_id(&self) -> &IssueId {
        &self.issue_id
    }

    pub fn version(&self) -> Version {
        self.version
    }
}

// TODO: tests
