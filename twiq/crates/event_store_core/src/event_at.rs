use std::{fmt::Display, str::FromStr};

use time::{format_description::well_known::Iso8601, OffsetDateTime, UtcOffset};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid format: {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventAt(OffsetDateTime);

impl Display for EventAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .format(&Iso8601::DEFAULT)
                .expect("format never fails")
        )
    }
}

impl EventAt {
    pub fn now() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}

impl FromStr for EventAt {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OffsetDateTime::parse(s, &Iso8601::DEFAULT)
            .map(|odt| odt.to_offset(UtcOffset::UTC))
            .map(Self)
            .map_err(|e| Error::InvalidFormat(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s_in_utc = "2020-01-02T03:04:05.006007008Z";
        let s_in_jst = "2020-01-02T12:04:05.006007008+09:00";
        let at = EventAt::now();
        assert_ne!(at.to_string(), s_in_utc);

        let at1 = EventAt::from_str(s_in_utc)?;
        assert_eq!(at1.to_string(), s_in_utc);

        let at2 = EventAt::from_str(s_in_jst)?;
        // always in utc
        assert_eq!(at2.to_string(), s_in_utc);

        assert_eq!(at1, at2);
        assert!(at > at1);
        Ok(())
    }
}
