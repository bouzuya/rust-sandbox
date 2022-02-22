use std::{fmt::Display, str::FromStr};

use thiserror::Error;

use crate::{IssueId, ParseIssueIdError};

#[derive(Debug, Error)]
pub enum ParseIssueBlockLinkError {
    #[error("InvalidFormat: {0}")]
    InvalidFormat(String),
    #[error("InvalidIssueId: {0}")]
    InvalidIssueId(#[from] ParseIssueIdError),
    #[error("InvalidBlockedId: {0}")]
    InvalidBlockedIssueId(IssueId),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueBlockLinkId(IssueId, IssueId);

impl Display for IssueBlockLinkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.0, self.1)
    }
}

impl FromStr for IssueBlockLinkId {
    type Err = ParseIssueBlockLinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split(" -> ").collect::<Vec<&str>>();
        if splitted.len() != 2 {
            return Err(Self::Err::InvalidFormat(s.to_string()));
        }
        let issue_id = IssueId::from_str(splitted[0])?;
        let blocked_issue_id = IssueId::from_str(splitted[1])?;
        Self::new(issue_id, blocked_issue_id)
    }
}

impl IssueBlockLinkId {
    pub fn new(
        issue_id: IssueId,
        blocked_issue_id: IssueId,
    ) -> Result<Self, ParseIssueBlockLinkError> {
        if issue_id == blocked_issue_id {
            return Err(ParseIssueBlockLinkError::InvalidBlockedIssueId(
                blocked_issue_id,
            ));
        }
        Ok(Self(issue_id, blocked_issue_id))
    }

    pub fn issue_id(&self) -> &IssueId {
        &self.0
    }

    pub fn blocked_issue_id(&self) -> &IssueId {
        &self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let issue_id = IssueId::from_str("1")?;
        let blocked_issue_id = IssueId::from_str("2")?;
        let issue_block_link = IssueBlockLinkId::new(issue_id.clone(), blocked_issue_id.clone())?;
        assert_eq!(issue_block_link.to_string(), "1 -> 2");
        assert_eq!(issue_block_link.issue_id(), &issue_id);
        assert_eq!(issue_block_link.blocked_issue_id(), &blocked_issue_id);
        let parsed = IssueBlockLinkId::from_str("1 -> 2")?;
        assert_eq!(issue_block_link, parsed);

        assert!(IssueBlockLinkId::from_str("1 -> 1").is_err());
        assert!(IssueBlockLinkId::from_str("1 -- 1").is_err());
        assert!(IssueBlockLinkId::from_str("a -> 1").is_err());
        assert!(IssueBlockLinkId::from_str("1 -> a").is_err());
        assert!(IssueBlockLinkId::from_str("1 -> 1 -> 1").is_err());
        Ok(())
    }
}
