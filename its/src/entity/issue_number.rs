use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueNumber(usize);

impl IssueNumber {
    pub fn start_number() -> Self {
        Self(1_usize)
    }

    pub fn next_number(&self) -> Self {
        if self.0 == std::usize::MAX {
            panic!("issue_number is overflow")
        }
        Self(self.0 + 1)
    }
}

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
    fn next_number_test() -> anyhow::Result<()> {
        let number1 = IssueNumber::try_from(1_usize)?;
        assert_eq!(number1.next_number(), IssueNumber::try_from(2_usize)?);
        Ok(())
    }

    #[test]
    fn start_number_test() -> anyhow::Result<()> {
        assert_eq!(IssueNumber::start_number(), IssueNumber::try_from(1_usize)?);
        Ok(())
    }

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert!(IssueNumber::from_str("a").is_err());
        assert!(IssueNumber::from_str("0").is_err());
        assert_eq!(IssueNumber::from_str("1")?, IssueNumber::try_from(1_usize)?);
        Ok(())
    }

    #[test]
    fn usize_conversion_test() -> anyhow::Result<()> {
        assert!(IssueNumber::try_from(0_usize).is_err());
        assert_eq!(usize::from(IssueNumber::try_from(1_usize)?), 1_usize);
        Ok(())
    }
}
