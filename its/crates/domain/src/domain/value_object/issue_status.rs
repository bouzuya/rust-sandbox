use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IssueStatus {
    Todo,
    Done,
}

impl std::fmt::Display for IssueStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IssueStatus::Todo => "todo",
                IssueStatus::Done => "done",
            }
        )
    }
}

impl std::str::FromStr for IssueStatus {
    type Err = ParseIssueStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "todo" => Ok(IssueStatus::Todo),
            "done" => Ok(IssueStatus::Done),
            _ => Err(ParseIssueStatusError {}),
        }
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
#[error("ParseIssueStatusError")]
pub struct ParseIssueStatusError {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert!(IssueStatus::from_str("in progress").is_err());
        assert_eq!(IssueStatus::from_str("todo")?, IssueStatus::Todo);
        assert_eq!(IssueStatus::from_str("done")?, IssueStatus::Done);
        Ok(())
    }
}
