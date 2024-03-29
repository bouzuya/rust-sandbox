use limited_date_time::Instant;

use crate::{aggregate::issue::attribute::IssueResolution, IssueId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueFinished {
    pub(crate) at: Instant,
    pub(crate) issue_id: IssueId,
    pub(crate) resolution: Option<IssueResolution>,
    pub(crate) version: Version,
}

impl IssueFinished {
    pub(crate) fn from_trusted_data(
        at: Instant,
        issue_id: IssueId,
        resolution: Option<IssueResolution>,
        version: Version,
    ) -> Self {
        Self::new(at, issue_id, resolution, version)
    }

    pub(crate) fn new(
        at: Instant,
        issue_id: IssueId,
        resolution: Option<IssueResolution>,
        version: Version,
    ) -> Self {
        Self {
            at,
            issue_id,
            resolution,
            version,
        }
    }

    pub fn at(&self) -> Instant {
        self.at
    }

    pub fn issue_id(&self) -> &IssueId {
        &self.issue_id
    }

    pub fn resolution(&self) -> Option<&IssueResolution> {
        self.resolution.as_ref()
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
        let resolution = IssueResolution::from_str("Duplicate")?;
        let version = Version::from(2_u64);
        let issue_finished = IssueFinished::from_trusted_data(
            at,
            issue_id.clone(),
            Some(resolution.clone()),
            version,
        );
        // TODO: new
        assert_eq!(issue_finished.at(), at);
        assert_eq!(issue_finished.issue_id(), &issue_id);
        assert_eq!(issue_finished.resolution(), Some(&resolution));
        assert_eq!(issue_finished.version(), version);
        Ok(())
    }
}
