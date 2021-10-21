use crate::{Date, ParseDateError, ParseTimeError, Time};

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateTime {
    date: Date,
    time: Time,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseDateTimeError {
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid length")]
    InvalidLength,
    #[error("parse date")]
    ParseDate(ParseDateError),
    #[error("parse time")]
    ParseTime(ParseTimeError),
}

impl DateTime {
    pub fn from_date_time(date: Date, time: Time) -> Self {
        Self { date, time }
    }

    pub fn date(&self) -> Date {
        self.date
    }

    pub fn time(&self) -> Time {
        self.time
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}T{}", self.date, self.time)
    }
}

impl std::str::FromStr for DateTime {
    type Err = ParseDateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 19 {
            return Err(Self::Err::InvalidLength);
        }
        let date = Date::from_str(&s[0..10]).map_err(ParseDateTimeError::ParseDate)?;
        if s.as_bytes().get(10) != Some(&b'T') {
            return Err(Self::Err::InvalidFormat);
        }
        let time = Time::from_str(&s[11..19]).map_err(ParseDateTimeError::ParseTime)?;
        Ok(DateTime { date, time })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn from_date_time_test() -> anyhow::Result<()> {
        let date = Date::from_str("2021-02-03")?;
        let time = Time::from_str("04:05:06")?;
        assert_eq!(
            DateTime::from_date_time(date, time),
            DateTime::from_str("2021-02-03T04:05:06")?
        );
        Ok(())
    }

    #[test]
    fn str_conversion_test() {
        type E = ParseDateTimeError;
        let f = |s: &str| DateTime::from_str(s);

        assert!(matches!(f("2021-02-03T04:05:06"), Ok(_)));
        assert!(matches!(f("20021-02-03T04:05:06"), Err(E::InvalidLength)));
        assert!(matches!(f("2021+02-03T04:05:06"), Err(E::ParseDate(_))));
        assert!(matches!(f("2021-02-03T04-05:06"), Err(E::ParseTime(_))));

        assert_eq!(
            f("2021-02-03T04:05:06").map(|d| d.to_string()),
            Ok("2021-02-03T04:05:06".to_string())
        );
    }

    #[test]
    fn date_test() -> anyhow::Result<()> {
        let date_time = DateTime::from_str("2021-02-03T04:05:06")?;
        assert_eq!(date_time.date(), Date::from_str("2021-02-03")?);
        Ok(())
    }

    #[test]
    fn time_test() -> anyhow::Result<()> {
        let date_time = DateTime::from_str("2021-02-03T04:05:06")?;
        assert_eq!(date_time.time(), Time::from_str("04:05:06")?);
        Ok(())
    }
}
