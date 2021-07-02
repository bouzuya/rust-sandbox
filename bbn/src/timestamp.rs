use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::{convert::TryInto, time::SystemTime};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn from_rfc3339(s: &str) -> anyhow::Result<Self> {
        Ok(Self(DateTime::parse_from_rfc3339(s)?.timestamp()))
    }

    pub fn now() -> anyhow::Result<Self> {
        Ok(Self(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs()
                .try_into()?,
        ))
    }

    pub fn to_rfc3339(&self) -> String {
        Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(self.0, 0))
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    }
}

impl From<i64> for Timestamp {
    fn from(timestamp: i64) -> Self {
        Self(timestamp)
    }
}

impl From<Timestamp> for i64 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i64_convert_test() {
        assert_eq!(i64::from(Timestamp::from(100_i64)), 100_i64);
    }

    #[test]
    fn string_convert_test() -> anyhow::Result<()> {
        assert_eq!(
            Timestamp::from_rfc3339("2021-07-03T08:46:05+09:00")?.to_rfc3339(),
            "2021-07-02T23:46:05Z"
        );
        Ok(())
    }
}
