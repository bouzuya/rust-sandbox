use std::{convert::TryFrom, str::FromStr};

use crate::{
    private::{date_time_string_from_timestamp, timestamp_from_date_time_string},
    DateTime, Instant, ParseDateTimeError, ParseTimeZoneOffsetError, TimeZoneOffset,
};

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OffsetDateTime {
    date_time: DateTime,
    offset: TimeZoneOffset,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseOffsetDateTimeError {
    #[error("invalid length")]
    InvalidLength,
    #[error("parse date time")]
    ParseDateTime(ParseDateTimeError),
    #[error("parse time zone offset")]
    ParseTimeZoneOffset(ParseTimeZoneOffsetError),
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromOffsetDateTimeError {
    #[error("out of range")]
    OutOfRange,
}

impl OffsetDateTime {
    pub fn new(date_time: DateTime, offset: TimeZoneOffset) -> Self {
        // FIXME: (1970-01-01T00:00:00 & -00:01) or (9999-12-31T00:00:00 & +00:01)
        Self { date_time, offset }
    }

    pub fn from_instant(
        instant: Instant,
        offset: TimeZoneOffset,
    ) -> Result<Self, TryFromOffsetDateTimeError> {
        let timestamp = i64::from(instant) + offset.offset_in_minutes() as i64 * 60;
        // check range
        Instant::try_from(timestamp).map_err(|_| TryFromOffsetDateTimeError::OutOfRange)?;
        let date_time = date_time_from_timestamp(timestamp);
        Ok(Self::new(date_time, offset))
    }

    pub fn instant(&self) -> Instant {
        Instant::from(*self)
    }

    pub fn date_time(&self) -> DateTime {
        self.date_time
    }

    pub fn offset(&self) -> TimeZoneOffset {
        self.offset
    }
}

impl std::fmt::Display for OffsetDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.date_time,
            if self.offset == TimeZoneOffset::utc() {
                "Z".to_string()
            } else {
                self.offset.to_string()
            }
        )
    }
}

impl std::str::FromStr for OffsetDateTime {
    type Err = ParseOffsetDateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 25 && s.len() != 20 {
            return Err(Self::Err::InvalidLength);
        }
        let date_time =
            DateTime::from_str(&s[0..19]).map_err(ParseOffsetDateTimeError::ParseDateTime)?;
        let offset = if s.len() == 25 {
            TimeZoneOffset::from_str(&s[19..25])
                .map_err(ParseOffsetDateTimeError::ParseTimeZoneOffset)
        } else if s.chars().nth(19) == Some('Z') {
            TimeZoneOffset::from_str("+00:00")
                .map_err(ParseOffsetDateTimeError::ParseTimeZoneOffset)
        } else {
            Err(ParseOffsetDateTimeError::ParseTimeZoneOffset(
                ParseTimeZoneOffsetError::InvalidFormat,
            ))
        }?;
        Ok(Self::new(date_time, offset))
    }
}

impl From<Instant> for OffsetDateTime {
    fn from(instant: Instant) -> Self {
        let date_time = date_time_from_timestamp(u64::from(instant) as i64);
        Self::new(date_time, TimeZoneOffset::utc())
    }
}

impl From<OffsetDateTime> for Instant {
    fn from(offset_date_time: OffsetDateTime) -> Self {
        // FIXME:
        let local_timestamp =
            timestamp_from_date_time_string(offset_date_time.date_time().to_string().as_str())
                .unwrap();
        let offset_in_seconds = offset_date_time.offset().offset_in_minutes() as i64 * 60;
        let utc_timestamp = local_timestamp - offset_in_seconds;
        Instant::try_from(utc_timestamp).expect("OffsetDateTime is broken")
    }
}

fn date_time_from_timestamp(timestamp: i64) -> DateTime {
    // FIXME:
    DateTime::from_str(date_time_string_from_timestamp(timestamp).as_ref().unwrap())
        .expect("date_time_string_from_timestamp")
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn instant_conversion_test() -> anyhow::Result<()> {
        let f = |s: &str, timestamp: u64| -> anyhow::Result<()> {
            let instant = Instant::try_from(timestamp)?;
            let offset_date_time = OffsetDateTime::from(instant);
            assert_eq!(offset_date_time.offset(), TimeZoneOffset::utc());
            assert_eq!(Instant::from(offset_date_time), instant);
            assert_eq!(offset_date_time.to_string(), s.to_string(),);
            Ok(())
        };
        f("1970-01-01T00:00:00Z", u64::from(Instant::min()))?;
        f("1970-01-02T00:00:01Z", 86401_u64)?;
        f("9999-12-31T23:59:59Z", u64::from(Instant::max()))?;

        {
            let instant = Instant::min();
            let offset = TimeZoneOffset::from_str("-00:01")?;
            assert!(OffsetDateTime::from_instant(instant, offset).is_err());
        }

        {
            let instant = Instant::min();
            let offset = TimeZoneOffset::utc();
            let offset_date_time = OffsetDateTime::from_instant(instant, offset)?;
            assert_eq!(offset_date_time.to_string(), "1970-01-01T00:00:00Z");
            assert_eq!(offset_date_time.instant(), instant);
        }
        {
            let instant = Instant::min();
            let offset = TimeZoneOffset::from_str("+00:01")?;
            let offset_date_time = OffsetDateTime::from_instant(instant, offset)?;
            assert_eq!(offset_date_time.to_string(), "1970-01-01T00:01:00+00:01");
            assert_eq!(offset_date_time.instant(), instant);
        }

        {
            let instant = Instant::try_from(i64::from(Instant::min()) + 60)?;
            let offset = TimeZoneOffset::from_str("-00:01")?;
            let offset_date_time = OffsetDateTime::from_instant(instant, offset)?;
            assert_eq!(offset_date_time.to_string(), "1970-01-01T00:00:00-00:01");
            assert_eq!(offset_date_time.instant(), instant);
        }

        {
            let instant = Instant::try_from(i64::from(Instant::max()) - 60)?;
            let offset = TimeZoneOffset::from_str("+00:01")?;
            let offset_date_time = OffsetDateTime::from_instant(instant, offset)?;
            assert_eq!(offset_date_time.to_string(), "9999-12-31T23:59:59+00:01");
            assert_eq!(offset_date_time.instant(), instant);
        }

        {
            let instant = Instant::max();
            let offset = TimeZoneOffset::from_str("-00:01")?;
            let offset_date_time = OffsetDateTime::from_instant(instant, offset)?;
            assert_eq!(offset_date_time.to_string(), "9999-12-31T23:58:59-00:01");
            assert_eq!(offset_date_time.instant(), instant);
        }

        {
            let instant = Instant::max();
            let offset = TimeZoneOffset::from_str("+00:00")?;
            let offset_date_time = OffsetDateTime::from_instant(instant, offset)?;
            assert_eq!(offset_date_time.to_string(), "9999-12-31T23:59:59Z");
            assert_eq!(offset_date_time.instant(), instant);
        }

        {
            let instant = Instant::max();
            let offset = TimeZoneOffset::from_str("+00:01")?;
            assert!(OffsetDateTime::from_instant(instant, offset).is_err());
        }
        Ok(())
    }

    #[test]
    fn str_conversion_test() {
        type E = ParseOffsetDateTimeError;
        let f = |s: &str| OffsetDateTime::from_str(s);

        assert!(matches!(f("2021-02-03T04:05:06+07:00"), Ok(_)));
        assert!(matches!(
            f("20021-02-03T04:05:06+07:00"),
            Err(E::InvalidLength)
        ));
        assert!(matches!(
            f("2021+02-03T04:05:06+07:00"),
            Err(E::ParseDateTime(_))
        ));
        assert!(matches!(
            f("2021-02-03T04:05:06+07-00"),
            Err(E::ParseTimeZoneOffset(_))
        ));

        assert_eq!(
            f("2021-02-03T04:05:06+07:00").map(|d| d.to_string()),
            Ok("2021-02-03T04:05:06+07:00".to_string())
        );
        assert_eq!(
            f("2021-02-03T04:05:06+00:00").map(|d| d.to_string()),
            Ok("2021-02-03T04:05:06Z".to_string())
        );
        assert_eq!(
            f("2021-02-03T04:05:06Z").map(|d| d.to_string()),
            Ok("2021-02-03T04:05:06Z".to_string())
        );
    }

    #[test]
    fn date_time_test() -> anyhow::Result<()> {
        let offset_date_time = OffsetDateTime::from_str("2021-02-03T04:05:06+07:00")?;
        assert_eq!(
            offset_date_time.date_time(),
            DateTime::from_str("2021-02-03T04:05:06")?
        );
        Ok(())
    }

    #[test]
    fn offset_test() -> anyhow::Result<()> {
        let offset_date_time = OffsetDateTime::from_str("2021-02-03T04:05:06+07:00")?;
        assert_eq!(
            offset_date_time.offset(),
            TimeZoneOffset::from_str("+07:00")?
        );
        Ok(())
    }
}
