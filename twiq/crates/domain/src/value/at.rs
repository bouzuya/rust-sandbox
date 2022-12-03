use std::{fmt::Display, str::FromStr};

use event_store_core::EventAt;
use time::{format_description::well_known::Iso8601, Duration, OffsetDateTime};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid precision {0}")]
    InvalidPrecision(String),
    #[error("error {0}")]
    Unknown(String),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct At(OffsetDateTime);

impl At {
    pub fn now() -> Self {
        Self(Self::truncate_nanosecond(OffsetDateTime::now_utc()))
    }

    pub fn plus_1day(&self) -> Self {
        Self(self.0 + Duration::days(1))
    }

    fn truncate_nanosecond(odt: OffsetDateTime) -> OffsetDateTime {
        odt.replace_nanosecond(odt.microsecond() * 1_000)
            .expect("nanosecond out of range")
    }
}

impl Display for At {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .format(&Iso8601::DEFAULT)
                .expect("invalid offset date time")
        )
    }
}

impl FromStr for At {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OffsetDateTime::parse(s, &Iso8601::DEFAULT)
            .map_err(|e| Error::Unknown(e.to_string()))
            .and_then(|odt| {
                if odt == Self::truncate_nanosecond(odt) {
                    Ok(odt)
                } else {
                    Err(Error::InvalidPrecision(odt.to_string()))
                }
            })
            .map(Self)
    }
}

impl TryFrom<String> for At {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<EventAt> for At {
    fn from(value: EventAt) -> Self {
        Self::from_str(&value.to_string()).expect("EventAt and At have same format")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        let s = "2020-01-02T03:04:05.123456000Z";
        assert_eq!(At::from_str(s)?.to_string(), s);
        assert!(At::from_str("2020-01-02T03:04:05.123456001Z").is_err());
        Ok(())
    }

    #[test]
    fn test_now() {
        let at = At::now();
        assert!(at.to_string().ends_with("000Z"));
        assert_ne!(at.to_string(), "2022-12-03T00:00:00.000000000Z");
    }

    #[test]
    fn plus_1day_test() -> anyhow::Result<()> {
        let at = At::from_str("2021-02-03T04:05:06.000000000Z")?;
        assert_eq!(at.plus_1day().to_string(), "2021-02-04T04:05:06.000000000Z");
        Ok(())
    }

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        let s = "2021-02-03T04:05:06.000000000Z";
        let at = At::from_str(s)?;
        assert_eq!(at.to_string(), s);
        let at = At::try_from(s.to_owned())?;
        assert_eq!(at.to_string(), s);
        Ok(())
    }
}
