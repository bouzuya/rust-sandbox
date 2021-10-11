use crate::{LocalDate, LocalTime, ParseLocalDateError, ParseLocalTimeError};

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LocalDateTime {
    date: LocalDate,
    time: LocalTime,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseLocalDateTimeError {
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid length")]
    InvalidLength,
    #[error("parse date")]
    ParseDate(ParseLocalDateError),
    #[error("parse time")]
    ParseTime(ParseLocalTimeError),
}

impl LocalDateTime {
    pub fn from_dt(date: LocalDate, time: LocalTime) -> Self {
        Self { date, time }
    }

    pub fn date(&self) -> LocalDate {
        self.date
    }

    pub fn time(&self) -> LocalTime {
        self.time
    }
}

impl std::fmt::Display for LocalDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}T{}", self.date, self.time)
    }
}

impl std::str::FromStr for LocalDateTime {
    type Err = ParseLocalDateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 19 {
            return Err(Self::Err::InvalidLength);
        }
        let date = LocalDate::from_str(&s[0..10]).map_err(ParseLocalDateTimeError::ParseDate)?;
        if s.as_bytes().get(10) != Some(&b'T') {
            return Err(Self::Err::InvalidFormat);
        }
        let time = LocalTime::from_str(&s[11..19]).map_err(ParseLocalDateTimeError::ParseTime)?;
        Ok(LocalDateTime { date, time })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn from_dt_test() -> anyhow::Result<()> {
        let date = LocalDate::from_str("2021-02-03")?;
        let time = LocalTime::from_str("04:05:06")?;
        assert_eq!(
            LocalDateTime::from_dt(date, time),
            LocalDateTime::from_str("2021-02-03T04:05:06")?
        );
        Ok(())
    }

    #[test]
    fn str_conversion_test() {
        type E = ParseLocalDateTimeError;
        let f = |s: &str| LocalDateTime::from_str(s);

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
        let dt = LocalDateTime::from_str("2021-02-03T04:05:06")?;
        assert_eq!(dt.date(), LocalDate::from_str("2021-02-03")?);
        Ok(())
    }

    #[test]
    fn time_test() -> anyhow::Result<()> {
        let dt = LocalDateTime::from_str("2021-02-03T04:05:06")?;
        assert_eq!(dt.time(), LocalTime::from_str("04:05:06")?);
        Ok(())
    }
}
