use limited_date_time::Instant;

use crate::{IssueBlockLinkId, IssueId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueUnblocked {
    pub(crate) at: Instant,
    pub(crate) issue_block_link_id: IssueBlockLinkId,
    pub(crate) version: Version,
}

impl IssueUnblocked {
    pub fn from_trusted_data(
        at: Instant,
        issue_block_link_id: IssueBlockLinkId,
        version: Version,
    ) -> Self {
        Self::new(at, issue_block_link_id, version)
    }

    pub(crate) fn new(
        at: Instant,
        issue_block_link_id: IssueBlockLinkId,
        version: Version,
    ) -> Self {
        Self {
            at,
            issue_block_link_id,
            version,
        }
    }

    pub fn at(&self) -> Instant {
        self.at
    }

    pub fn blocked_issue_id(&self) -> &IssueId {
        self.issue_block_link_id.blocked_issue_id()
    }

    pub fn issue_id(&self) -> &IssueId {
        self.issue_block_link_id.issue_id()
    }

    pub fn key(&self) -> (&IssueBlockLinkId, Version) {
        (&self.issue_block_link_id, self.version)
    }

    pub fn version(&self) -> Version {
        self.version
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let at = Instant::now();
        let issue_block_link_id = IssueBlockLinkId::from_str("1 -> 2")?;
        let version = Version::from(1_u64);
        let issue_unblocked =
            IssueUnblocked::from_trusted_data(at, issue_block_link_id.clone(), version);
        assert_eq!(
            IssueUnblocked::new(at, issue_block_link_id.clone(), version),
            issue_unblocked
        );
        assert_eq!(issue_unblocked.at(), at);
        assert_eq!(
            issue_unblocked.blocked_issue_id(),
            issue_block_link_id.blocked_issue_id()
        );
        assert_eq!(issue_unblocked.issue_id(), issue_block_link_id.issue_id());
        assert_eq!(issue_unblocked.key(), (&issue_block_link_id, version));
        assert_eq!(issue_unblocked.version(), version);
        Ok(())
    }
}
