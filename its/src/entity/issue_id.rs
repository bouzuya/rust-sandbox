use thiserror::Error;

use super::IssueNumber;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueId(IssueNumber);

impl IssueId {
    pub fn new(issue_number: IssueNumber) -> Self {
        Self(issue_number)
    }

    pub fn issue_number(&self) -> IssueNumber {
        self.0
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
#[error("ParseIssueIdError")]
pub struct ParseIssueIdError {}

impl std::str::FromStr for IssueId {
    type Err = ParseIssueIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let issue_number = IssueNumber::from_str(s).map_err(|_| ParseIssueIdError {})?;
        Ok(Self(issue_number))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert!(IssueId::from_str("a").is_err());
        assert!(IssueId::from_str("0").is_err());
        assert_eq!(
            IssueId::from_str("1")?,
            IssueId::new(IssueNumber::from_str("1")?)
        );
        Ok(())
    }
}
