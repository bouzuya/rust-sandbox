use std::{fmt::Display, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("from str error")]
    FromStr,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Value {
    InProgress,
    Completed,
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "in_progress" => Ok(Value::InProgress),
            "completed" => Ok(Value::Completed),
            _ => Err(Error::FromStr),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::InProgress => "in_progress",
                Value::Completed => "completed",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert_eq!(Value::from_str("in_progress")?, Value::InProgress);
        assert_eq!(Value::InProgress.to_string(), "in_progress");
        assert_eq!(Value::from_str("completed")?, Value::Completed);
        assert_eq!(Value::Completed.to_string(), "completed");
        Ok(())
    }
}
