use limited_date_time::Instant;

use crate::{IssueBlockLinkId, IssueId, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueBlocked {
    pub(crate) at: Instant,
    pub(crate) issue_block_link_id: IssueBlockLinkId,
    pub(crate) version: Version,
}

impl IssueBlocked {
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
        let issue_blocked =
            IssueBlocked::from_trusted_data(at, issue_block_link_id.clone(), version);
        assert_eq!(
            IssueBlocked::new(at, issue_block_link_id.clone(), version),
            issue_blocked
        );
        assert_eq!(issue_blocked.at(), at);
        assert_eq!(
            issue_blocked.blocked_issue_id(),
            issue_block_link_id.blocked_issue_id()
        );
        assert_eq!(issue_blocked.issue_id(), issue_block_link_id.issue_id());
        assert_eq!(issue_blocked.key(), (&issue_block_link_id, version));
        assert_eq!(issue_blocked.version(), version);
        Ok(())
    }
}
