use crate::{IssueBlockLinkId, IssueBlockLinkStatus, IssueId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IssueBlockLink {
    id: IssueBlockLinkId,
    status: IssueBlockLinkStatus,
}

impl IssueBlockLink {
    pub(crate) fn new(id: IssueBlockLinkId) -> Self {
        Self {
            id,
            status: IssueBlockLinkStatus::Blocked,
        }
    }

    pub(crate) fn issue_id(&self) -> &IssueId {
        self.id.issue_id()
    }

    pub(crate) fn blocked_issue_id(&self) -> &IssueId {
        self.id.blocked_issue_id()
    }

    pub(crate) fn is_blocked(&self) -> bool {
        self.status == IssueBlockLinkStatus::Blocked
    }

    pub(crate) fn block(&self) -> Self {
        Self {
            id: self.id.clone(),
            status: IssueBlockLinkStatus::Blocked,
        }
    }

    pub(crate) fn unblock(&self) -> Self {
        Self {
            id: self.id.clone(),
            status: IssueBlockLinkStatus::Unblocked,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let id = IssueBlockLinkId::from_str("1 -> 2")?;
        let link = IssueBlockLink::new(id.clone());
        assert_eq!(link.issue_id(), id.issue_id());
        assert_eq!(link.blocked_issue_id(), id.blocked_issue_id());
        assert!(link.is_blocked());

        let unblocked = link.unblock();
        assert!(!unblocked.is_blocked());

        let blocked = unblocked.block();
        assert!(blocked.is_blocked());

        Ok(())
    }
}
