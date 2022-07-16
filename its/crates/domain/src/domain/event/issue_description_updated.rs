use limited_date_time::Instant;

use crate::{aggregate::issue::IssueDescription, IssueId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueDescriptionUpdated {
    pub(crate) at: Instant,
    pub(crate) issue_id: IssueId,
    pub(crate) issue_description: IssueDescription,
    pub(crate) version: Version,
}

impl IssueDescriptionUpdated {
    pub(crate) fn from_trusted_data(
        at: Instant,
        issue_id: IssueId,
        issue_description: IssueDescription,
        version: Version,
    ) -> Self {
        Self::new(at, issue_id, issue_description, version)
    }

    pub(crate) fn new(
        at: Instant,
        issue_id: IssueId,
        issue_description: IssueDescription,
        version: Version,
    ) -> Self {
        Self {
            at,
            issue_id,
            issue_description,
            version,
        }
    }

    pub fn at(&self) -> Instant {
        self.at
    }

    pub fn issue_id(&self) -> &IssueId {
        &self.issue_id
    }

    pub fn issue_description(&self) -> &IssueDescription {
        &self.issue_description
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
        let issue_description = IssueDescription::from_str("desc2")?;
        let version = Version::from(2_u64);
        let issue_updated = IssueDescriptionUpdated::from_trusted_data(
            at,
            issue_id.clone(),
            issue_description.clone(),
            version,
        );
        // TODO: new
        assert_eq!(issue_updated.at(), at);
        assert_eq!(issue_updated.issue_id(), &issue_id);
        assert_eq!(issue_updated.issue_description(), &issue_description);
        assert_eq!(issue_updated.version(), version);
        Ok(())
    }
}
