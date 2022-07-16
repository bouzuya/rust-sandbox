use thiserror::Error;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueResolution(String);

#[derive(Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error("invalid length {0}")]
    InvalidLength(usize),
}

impl std::fmt::Display for IssueResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for IssueResolution {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

impl TryFrom<String> for IssueResolution {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 255 {
            Err(Error::InvalidLength(value.len()))
        } else {
            Ok(Self(value))
        }
    }
}
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert!(IssueResolution::from_str("a".repeat(256).as_str()).is_err());
        assert!(IssueResolution::from_str("a".repeat(255).as_str()).is_ok());
        assert_eq!(
            IssueResolution::from_str("a".repeat(255).as_str())?.to_string(),
            "a".repeat(255)
        );

        assert!(IssueResolution::try_from("a".repeat(256)).is_err());
        assert!(IssueResolution::try_from("a".repeat(255)).is_ok());
        assert_eq!(
            IssueResolution::try_from("a".repeat(255))?.to_string(),
            "a".repeat(255)
        );
        Ok(())
    }
}
