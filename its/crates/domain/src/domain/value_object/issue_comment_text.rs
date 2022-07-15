#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueCommentText(String);

impl std::fmt::Display for IssueCommentText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("invalid length {0}")]
    InvalidLength(usize),
}

impl std::str::FromStr for IssueCommentText {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

impl TryFrom<String> for IssueCommentText {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 255 {
            Err(Error::InvalidLength(value.len()))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<IssueCommentText> for String {
    fn from(text: IssueCommentText) -> Self {
        text.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn default_test() -> anyhow::Result<()> {
        assert_eq!(IssueCommentText::default(), IssueCommentText::from_str("")?);
        Ok(())
    }

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert!(IssueCommentText::from_str("a".repeat(256).as_str()).is_err());
        assert!(IssueCommentText::from_str("a".repeat(255).as_str()).is_ok());
        assert_eq!(
            IssueCommentText::from_str("a".repeat(255).as_str())?.to_string(),
            "a".repeat(255)
        );
        assert!(IssueCommentText::try_from("a".repeat(256)).is_err());
        assert!(IssueCommentText::try_from("a".repeat(255)).is_ok());
        assert_eq!(
            String::from(IssueCommentText::try_from("a".repeat(255))?),
            "a".repeat(255)
        );
        Ok(())
    }
}
