use limited_date_time::Instant;

use crate::{aggregate::issue::attribute::IssueDue, IssueId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueUpdated {
    pub(crate) at: Instant,
    pub(crate) issue_id: IssueId,
    pub(crate) issue_due: Option<IssueDue>,
    pub(crate) version: Version,
}

impl IssueUpdated {
    pub(crate) fn from_trusted_data(
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::IssueNumber;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_id = IssueId::new(IssueNumber::start_number());
        let issue_due = IssueDue::from_str("2021-02-03T04:05:06Z")?;
        let version = Version::from(2_u64);
        let issue_updated =
            IssueUpdated::from_trusted_data(at, issue_id.clone(), Some(issue_due), version);
        // TODO: new
        assert_eq!(issue_updated.at(), at);
        assert_eq!(issue_updated.issue_id(), &issue_id);
        assert_eq!(issue_updated.issue_due(), Some(issue_due));
        assert_eq!(issue_updated.version(), version);
        Ok(())
    }
}
