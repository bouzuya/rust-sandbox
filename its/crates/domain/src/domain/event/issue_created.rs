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
    pub(crate) fn from_trusted_data(
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

#[cfg(test)]
mod tests {
    use crate::IssueNumber;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_id = IssueId::new(IssueNumber::start_number());
        let issue_title = IssueTitle::try_from("title".to_string())?;
        let version = Version::from(2_u64);
        let issue_updated =
            IssueCreated::from_trusted_data(at, issue_id.clone(), issue_title.clone(), version);
        // TODO: new
        assert_eq!(issue_updated.at(), at);
        assert_eq!(issue_updated.issue_id(), &issue_id);
        assert_eq!(issue_updated.issue_title(), &issue_title);
        assert_eq!(issue_updated.version(), version);
        Ok(())
    }
}
