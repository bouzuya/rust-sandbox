use limited_date_time::{Instant, OffsetDateTime};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Timestamp(Instant);

impl Timestamp {
    pub fn from_rfc3339(s: &str) -> anyhow::Result<Self> {
        Ok(Self(OffsetDateTime::from_str(s)?.instant()))
    }

    pub fn now() -> anyhow::Result<Self> {
        Ok(Self(Instant::now()))
    }

    pub fn to_rfc3339(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<i64> for Timestamp {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(Instant::try_from(value)?))
    }
}

impl From<Timestamp> for i64 {
    fn from(timestamp: Timestamp) -> Self {
        i64::from(timestamp.0)
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
