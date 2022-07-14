use std::{fmt::Display, str::FromStr};

use ulid::Ulid;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid format : {0}")]
    InvalidFormat(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueCommentId(Ulid);

impl Display for IssueCommentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for IssueCommentId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            Ulid::from_str(s).map_err(|e| Error::InvalidFormat(e.to_string()))?,
        ))
    }
}

impl IssueCommentId {
    pub fn generate() -> Self {
        Self(Ulid::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "01G7ZBK0V6CC6KF8AHXY3Z6S91";
        let issue_comment_id = IssueCommentId::from_str(s)?;
        assert_eq!(issue_comment_id.to_string(), s);
        assert_ne!(IssueCommentId::generate(), issue_comment_id);
        Ok(())
    }
}
