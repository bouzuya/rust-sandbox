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
        Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(self.0, 0).unwrap())
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    }
}

impl TryFrom<i64> for Timestamp {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if !(0..=253_402_300_799).contains(&value) {
            return Err(anyhow::anyhow!("timestamp out of range"));
        }
        Ok(Self(value))
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
    fn i64_convert_test() -> anyhow::Result<()> {
        assert!(Timestamp::try_from(0_i64 - 1).is_err());
        assert_eq!(i64::from(Timestamp::try_from(0_i64)?), 0_i64);
        assert_eq!(
            Timestamp::try_from(0_i64)?.to_rfc3339(),
            "1970-01-01T00:00:00Z"
        );
        assert_eq!(
            i64::from(Timestamp::try_from(253_402_300_799_i64)?),
            253_402_300_799_i64
        );
        assert_eq!(
            Timestamp::try_from(253_402_300_799_i64)?.to_rfc3339(),
            "9999-12-31T23:59:59Z"
        );
        assert!(Timestamp::try_from(253_402_300_799_i64 + 1).is_err());
        Ok(())
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
