use std::convert::TryFrom;
use thiserror::Error;

use super::{DayOfWeek, ParseWeekError, ParseWeekYearError, Week, WeekYear};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct YearWeek {
    year: WeekYear,
    week: Week,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseYearWeekError {
    #[error("invalid length")]
    InvalidLength,
    #[error("invalid format")]
    InvalidFormat,
    #[error("parse year")]
    ParseWeekYear(ParseWeekYearError),
    #[error("parse week")]
    ParseWeek(ParseWeekError),
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromYearWeekError {
    #[error("out of range")]
    OutOfRange,
}

impl YearWeek {
    pub fn from_yw(year: WeekYear, week: Week) -> Result<Self, TryFromYearWeekError> {
        if week > year.last_week() {
            return Err(TryFromYearWeekError::OutOfRange);
        }
        Ok(Self { year, week })
    }

    pub fn first_day_of_week(&self) -> DayOfWeek {
        if u16::from(self.year) == 1970 && u8::from(self.week) == 1 {
            DayOfWeek::try_from(4).unwrap()
        } else {
            DayOfWeek::try_from(1).unwrap()
        }
    }

    pub fn last_day_of_week(&self) -> DayOfWeek {
        if u16::from(self.year) == 9999 && self.year.last_week() == self.week {
            DayOfWeek::try_from(5).unwrap()
        } else {
            DayOfWeek::try_from(7).unwrap()
        }
    }

    pub fn week(&self) -> Week {
        self.week
    }

    pub fn year(&self) -> WeekYear {
        self.year
    }
}

impl std::fmt::Display for YearWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-W{}", self.year, self.week)
    }
}

impl std::str::FromStr for YearWeek {
    type Err = ParseYearWeekError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 01234567
        // YYYY-Www
        if s.len() != 8 {
            return Err(Self::Err::InvalidLength);
        }
        if s.as_bytes().get(4) != Some(&b'-') || s.as_bytes().get(5) != Some(&b'W') {
            return Err(Self::Err::InvalidFormat);
        }
        let year = match WeekYear::from_str(&s[0..4]) {
            Ok(y) => y,
            Err(e) => return Err(Self::Err::ParseWeekYear(e)),
        };
        let week = match Week::from_str(&s[6..8]) {
            Ok(m) => m,
            Err(e) => return Err(Self::Err::ParseWeek(e)),
        };
        Ok(Self { year, week })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn from_yw() {
        let y = |s| WeekYear::from_str(s).unwrap();
        let w = |s| Week::from_str(s).unwrap();
        let yw = |y, w| YearWeek::from_yw(y, w);
        assert_eq!(
            yw(y("2021"), w("01")).unwrap(),
            YearWeek::from_str("2021-W01").unwrap()
        );
        assert!(matches!(
            yw(y("2021"), w("53")),
            Err(TryFromYearWeekError::OutOfRange)
        ));
    }

    #[test]
    fn year() {
        assert_eq!(
            YearWeek::from_str("2021-W01").unwrap().year(),
            WeekYear::from_str("2021").unwrap()
        );
    }

    #[test]
    fn week() {
        assert_eq!(
            YearWeek::from_str("2021-W01").unwrap().week(),
            Week::from_str("01").unwrap()
        );
    }

    #[test]
    fn str_convert() {
        let f = |s| YearWeek::from_str(s);
        type PYE = ParseYearWeekError;
        assert_eq!(
            f("2000-W01").map(|yw| yw.to_string()),
            Ok("2000-W01".to_string())
        );
        assert!(matches!(f("20000-W01"), Err(PYE::InvalidLength)));
        assert!(matches!(f("2000+W01"), Err(PYE::InvalidFormat)));
        assert!(matches!(f("+000-W01"), Err(PYE::ParseWeekYear(_))));
        assert!(matches!(f("2000-W54"), Err(PYE::ParseWeek(_))));
    }
}
