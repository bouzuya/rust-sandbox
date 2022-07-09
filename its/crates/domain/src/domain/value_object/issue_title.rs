#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueTitle(String);

impl std::fmt::Display for IssueTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("invalid length {0}")]
    InvalidLength(usize),
}

impl std::str::FromStr for IssueTitle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

impl TryFrom<String> for IssueTitle {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 255 {
            Err(Error::InvalidLength(value.len()))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<IssueTitle> for String {
    fn from(issue_title: IssueTitle) -> Self {
        issue_title.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert!(IssueTitle::from_str("a".repeat(256).as_str()).is_err());
        assert!(IssueTitle::from_str("a".repeat(255).as_str()).is_ok());
        assert_eq!(
            IssueTitle::from_str("a".repeat(255).as_str())?.to_string(),
            "a".repeat(255)
        );
        assert!(IssueTitle::try_from("a".repeat(256)).is_err());
        assert!(IssueTitle::try_from("a".repeat(255)).is_ok());
        assert_eq!(
            String::from(IssueTitle::try_from("a".repeat(255))?),
            "a".repeat(255)
        );
        Ok(())
    }
}
