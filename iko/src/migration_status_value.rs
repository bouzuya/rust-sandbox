use std::{fmt::Display, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("from str error")]
    FromStr,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MigrationStatusValue {
    InProgress,
    Completed,
}

impl FromStr for MigrationStatusValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "in_progress" => Ok(MigrationStatusValue::InProgress),
            "completed" => Ok(MigrationStatusValue::Completed),
            _ => Err(Error::FromStr),
        }
    }
}

impl Display for MigrationStatusValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MigrationStatusValue::InProgress => "in_progress",
                MigrationStatusValue::Completed => "completed",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert_eq!(
            MigrationStatusValue::from_str("in_progress")?,
            MigrationStatusValue::InProgress
        );
        assert_eq!(MigrationStatusValue::InProgress.to_string(), "in_progress");
        assert_eq!(
            MigrationStatusValue::from_str("completed")?,
            MigrationStatusValue::Completed
        );
        assert_eq!(MigrationStatusValue::Completed.to_string(), "completed");
        Ok(())
    }
}
