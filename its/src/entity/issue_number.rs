use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueNumber(usize);

#[derive(Debug, Eq, Error, PartialEq)]
#[error("ParseIssueNumberError")]
pub struct ParseIssueNumberError {}

#[derive(Debug, Eq, Error, PartialEq)]
#[error("TryFromIssueNumberError")]
pub struct TryFromIssueNumberError {}

impl std::str::FromStr for IssueNumber {
    type Err = ParseIssueNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = usize::from_str(s).map_err(|_| ParseIssueNumberError {})?;
        Self::try_from(value).map_err(|_| ParseIssueNumberError {})
    }
}

impl TryFrom<usize> for IssueNumber {
    type Error = TryFromIssueNumberError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(TryFromIssueNumberError {})
        } else {
            Ok(Self(value))
        }
    }
}

impl From<IssueNumber> for usize {
    fn from(issue_number: IssueNumber) -> Self {
        issue_number.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn usize_conversion_test() -> anyhow::Result<()> {
        assert!(IssueNumber::try_from(0_usize).is_err());
        assert_eq!(usize::from(IssueNumber::try_from(1_usize)?), 1_usize);
        Ok(())
    }

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert!(IssueNumber::from_str("a").is_err());
        assert!(IssueNumber::from_str("0").is_err());
        assert_eq!(IssueNumber::from_str("1")?, IssueNumber::try_from(1_usize)?);
        Ok(())
    }
}
