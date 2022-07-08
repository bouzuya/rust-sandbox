use limited_date_time::{Instant, OffsetDateTime, ParseOffsetDateTimeError};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueDue(Instant);

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("invalid format {0}")]
    InvalidFormat(#[from] ParseOffsetDateTimeError),
}

impl std::fmt::Display for IssueDue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for IssueDue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = OffsetDateTime::from_str(s)?;
        Ok(Self::from(value.instant()))
    }
}

impl From<Instant> for IssueDue {
    fn from(value: Instant) -> Self {
        Self(value)
    }
}

impl From<IssueDue> for Instant {
    fn from(issue_due: IssueDue) -> Self {
        issue_due.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert!(IssueDue::from_str("a").is_err());
        assert!(IssueDue::from_str("0").is_err());
        assert_eq!(
            IssueDue::from_str("2021-02-03T04:05:06Z")?,
            IssueDue::try_from(Instant::from_str("2021-02-03T04:05:06Z")?)?
        );
        assert_eq!(
            IssueDue::from_str("2021-02-03T04:05:06Z")?.to_string(),
            "2021-02-03T04:05:06Z"
        );
        Ok(())
    }

    #[test]
    fn instant_conversion_test() -> anyhow::Result<()> {
        let instant = Instant::from_str("2021-02-03T04:05:06Z")?;
        assert_eq!(Instant::from(IssueDue::from(instant)), instant);
        Ok(())
    }
}
