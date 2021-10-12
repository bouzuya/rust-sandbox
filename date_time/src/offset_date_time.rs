use crate::{LocalDateTime, ParseLocalDateTimeError, ParseTimeZoneOffsetError, TimeZoneOffset};

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
        Self {
            local_date_time,
            time_zone_offset,
        }
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
            if self.time_zone_offset.hour() == 0 && self.time_zone_offset.minute() == 0 {
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
        Ok(OffsetDateTime {
            local_date_time,
            time_zone_offset,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

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
