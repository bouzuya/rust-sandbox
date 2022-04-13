use thiserror::Error;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueResolution(String);

#[derive(Debug, Eq, Error, PartialEq)]
#[error("ParseIssueResolutionError")]
pub struct ParseIssueResolutionError {}

impl std::fmt::Display for IssueResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for IssueResolution {
    type Err = ParseIssueResolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert_eq!(IssueResolution::from_str("a")?.to_string(), "a");
        Ok(())
    }
}
