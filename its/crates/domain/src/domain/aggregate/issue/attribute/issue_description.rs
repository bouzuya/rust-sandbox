#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueDescription(String);

impl std::fmt::Display for IssueDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("invalid length {0}")]
    InvalidLength(usize),
}

impl std::str::FromStr for IssueDescription {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

impl TryFrom<String> for IssueDescription {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 255 {
            Err(Error::InvalidLength(value.len()))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<IssueDescription> for String {
    fn from(issue_description: IssueDescription) -> Self {
        issue_description.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn default_test() -> anyhow::Result<()> {
        assert_eq!(IssueDescription::default(), IssueDescription::from_str("")?);
        Ok(())
    }

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert!(IssueDescription::from_str("a".repeat(256).as_str()).is_err());
        assert!(IssueDescription::from_str("a".repeat(255).as_str()).is_ok());
        assert_eq!(
            IssueDescription::from_str("a".repeat(255).as_str())?.to_string(),
            "a".repeat(255)
        );
        assert!(IssueDescription::try_from("a".repeat(256)).is_err());
        assert!(IssueDescription::try_from("a".repeat(255)).is_ok());
        assert_eq!(
            String::from(IssueDescription::try_from("a".repeat(255))?),
            "a".repeat(255)
        );
        Ok(())
    }
}
