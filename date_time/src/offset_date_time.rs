use std::{convert::TryFrom, str::FromStr};

use crate::{
    Instant, LocalDateTime, ParseLocalDateTimeError, ParseTimeZoneOffsetError, TimeZoneOffset,
};

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OffsetDateTime {
    local_date_time: LocalDateTime,
    time_zone_offset: TimeZoneOffset,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseOffsetDateTimeError {
    #[error("invalid length")]
    InvalidLength,
    #[error("parse local date time")]
    ParseLocalDateTime(ParseLocalDateTimeError),
    #[error("parse time zone offset")]
    ParseTimeZoneOffset(ParseTimeZoneOffsetError),
}

impl OffsetDateTime {
    pub fn new(local_date_time: LocalDateTime, time_zone_offset: TimeZoneOffset) -> Self {
        // FIXME: (1970-01-01T00:00:00 & -00:01) or (9999-12-31T00:00:00 & +00:01)
        Self {
            local_date_time,
            time_zone_offset,
        }
    }

    pub fn from_instant(instant: Instant, time_zone_offset: TimeZoneOffset) -> Self {
        // FIXME: (1970-01-01T00:00:00 & -00:01) or (9999-12-31T00:00:00 & +00:01)
        let timestamp =
            (u64::from(instant) as i64 + time_zone_offset.offset_in_minutes() as i64 * 60) as u64;
        let local_date_time = local_date_time_from_instant(Instant::try_from(timestamp).unwrap());
        Self {
            local_date_time,
            time_zone_offset,
        }
    }

    pub fn instant(&self) -> Instant {
        Instant::from(*self)
    }

    pub fn local_date_time(&self) -> LocalDateTime {
        self.local_date_time
    }

    pub fn time_zone_offset(&self) -> TimeZoneOffset {
        self.time_zone_offset
    }
}

impl std::fmt::Display for OffsetDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.local_date_time,
            if self.time_zone_offset == TimeZoneOffset::utc() {
                "Z".to_string()
            } else {
                self.time_zone_offset.to_string()
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
        let local_date_time = LocalDateTime::from_str(&s[0..19])
            .map_err(ParseOffsetDateTimeError::ParseLocalDateTime)?;
        let time_zone_offset = if s.len() == 25 {
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
        Ok(Self::new(local_date_time, time_zone_offset))
    }
}

impl From<Instant> for OffsetDateTime {
    fn from(instant: Instant) -> Self {
        let local_date_time = local_date_time_from_instant(instant);
        Self::new(local_date_time, TimeZoneOffset::utc())
    }
}

impl From<OffsetDateTime> for Instant {
    fn from(offset_date_time: OffsetDateTime) -> Self {
        let local_timestamp = instant_from_local_date_time(offset_date_time.local_date_time());
        let offset_in_seconds = offset_date_time.time_zone_offset().offset_in_minutes() as i64 * 60;
        let utc_timestamp = (local_timestamp as i64 - offset_in_seconds) as u64;
        Instant::try_from(utc_timestamp).expect("OffsetDateTime is broken")
    }
}

fn local_date_time_from_instant(instant: Instant) -> LocalDateTime {
    use chrono::NaiveDateTime;

    let timestamp = u64::from(instant) as i64;
    let naive_date_time = NaiveDateTime::from_timestamp(timestamp, 0);
    LocalDateTime::from_str(&format!("{:?}", naive_date_time))
        .expect("unexpected NaiveDateTime debug format")
}

fn instant_from_local_date_time(local_date_time: LocalDateTime) -> u64 {
    use chrono::NaiveDateTime;

    NaiveDateTime::from_str(&local_date_time.to_string())
        .expect("unexpected NaiveDateTime::from_str")
        .timestamp() as u64
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
            assert_eq!(offset_date_time.time_zone_offset(), TimeZoneOffset::utc());
            assert_eq!(Instant::from(offset_date_time), instant);
            assert_eq!(offset_date_time.to_string(), s.to_string(),);
            Ok(())
        };
        f("1970-01-01T00:00:00Z", 0)?;
        f("1970-01-02T00:00:01Z", 86401)?;
        f("9999-12-31T23:59:59Z", 253_402_300_799_u64)?;

        {
            let instant = Instant::try_from(0_u64)?;
            let time_zone_offset = TimeZoneOffset::from_str("+09:00")?;
            let offset_date_time = OffsetDateTime::from_instant(instant, time_zone_offset);
            assert_eq!(offset_date_time.to_string(), "1970-01-01T09:00:00+09:00");
            assert_eq!(offset_date_time.instant(), instant);
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
            Err(E::ParseLocalDateTime(_))
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
    fn local_date_time_test() -> anyhow::Result<()> {
        let dt = OffsetDateTime::from_str("2021-02-03T04:05:06+07:00")?;
        assert_eq!(
            dt.local_date_time(),
            LocalDateTime::from_str("2021-02-03T04:05:06")?
        );
        Ok(())
    }

    #[test]
    fn time_zone_offset_test() -> anyhow::Result<()> {
        let dt = OffsetDateTime::from_str("2021-02-03T04:05:06+07:00")?;
        assert_eq!(dt.time_zone_offset(), TimeZoneOffset::from_str("+07:00")?);
        Ok(())
    }
}
