mod day_of_week;
mod week;
mod week_year;
mod year_week;

use std::convert::TryFrom;

use crate::date::Date;

pub use self::day_of_week::{DayOfWeek, ParseDayOfWeekError};
pub use self::week::{ParseWeekError, Week};
pub use self::week_year::{ParseWeekYearError, WeekYear};
pub use self::year_week::{ParseYearWeekError, YearWeek};
use chrono::Datelike;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WeekDate {
    year: WeekYear,
    week: Week,
    day_of_week: DayOfWeek,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseWeekDateError {
    #[error("invalid day of week")]
    InvalidDayOfWeek,
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid length")]
    InvalidLength,
    #[error("parse day of week")]
    ParseDayOfWeek(ParseDayOfWeekError),
    #[error("parse week")]
    ParseWeek(ParseWeekError),
    #[error("parse year")]
    ParseYear(ParseWeekYearError),
}

impl WeekDate {
    pub fn day_of_week(&self) -> DayOfWeek {
        self.day_of_week
    }

    pub fn week(&self) -> Week {
        self.week
    }

    pub fn year(&self) -> WeekYear {
        self.year
    }

    pub fn year_week(&self) -> YearWeek {
        YearWeek::from_yw(self.year, self.week).expect("internal error")
    }
}

impl std::fmt::Display for WeekDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-W{}-{}", self.year, self.week, self.day_of_week)
    }
}

impl std::str::FromStr for WeekDate {
    type Err = ParseWeekDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 0123456789
        // 2021-W01-1
        if s.len() != 10 {
            return Err(Self::Err::InvalidLength);
        }
        let year_week = match YearWeek::from_str(&s[0..8]) {
            Ok(yw) => yw,
            Err(e) => match e {
                ParseYearWeekError::InvalidLength => unreachable!(),
                ParseYearWeekError::InvalidFormat => return Err(Self::Err::InvalidFormat),
                ParseYearWeekError::ParseWeekYear(e) => return Err(Self::Err::ParseYear(e)),
                ParseYearWeekError::ParseWeek(e) => return Err(Self::Err::ParseWeek(e)),
            },
        };
        if s.as_bytes().get(8) != Some(&b'-') {
            return Err(Self::Err::InvalidFormat);
        }
        let day_of_week = match DayOfWeek::from_str(&s[9..10]) {
            Ok(d) => d,
            Err(e) => return Err(Self::Err::ParseDayOfWeek(e)),
        };
        if day_of_week < year_week.first_day_of_week() || day_of_week > year_week.last_day_of_week()
        {
            return Err(Self::Err::InvalidDayOfWeek);
        }
        Ok(WeekDate {
            year: year_week.year(),
            week: year_week.week(),
            day_of_week,
        })
    }
}

impl From<Date> for WeekDate {
    // TODO:
    fn from(date: Date) -> Self {
        let y = u16::from(date.year());
        let m = u8::from(date.month());
        let d = u8::from(date.day_of_month());
        let date = chrono::NaiveDate::from_ymd(y as i32, m as u32, d as u32);
        WeekDate {
            year: WeekYear::try_from(date.iso_week().year() as u16).expect("internal error"),
            week: Week::try_from(date.iso_week().week() as u8).expect("internal error"),
            day_of_week: DayOfWeek::try_from(date.weekday().number_from_monday() as u8)
                .expect("internal error"),
        }
    }
}

impl From<WeekDate> for Date {
    // TODO:
    fn from(week_date: WeekDate) -> Self {
        let y = u16::from(week_date.year());
        let w = u8::from(week_date.week());
        let wd = u8::from(week_date.day_of_week());
        let date = chrono::NaiveDate::from_isoywd(
            y as i32,
            w as u32,
            match wd {
                1 => chrono::Weekday::Mon,
                2 => chrono::Weekday::Tue,
                3 => chrono::Weekday::Wed,
                4 => chrono::Weekday::Thu,
                5 => chrono::Weekday::Fri,
                6 => chrono::Weekday::Sat,
                7 => chrono::Weekday::Sun,
                _ => unreachable!(),
            },
        );
        format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day())
            .parse()
            .expect("internal error")
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn date_convert() {
        let d = |s: &str| Date::from_str(s).unwrap();
        let f = |s: &str| Date::from(WeekDate::from_str(s).unwrap());

        assert_eq!(f("2020-W53-5"), d("2021-01-01"));
        assert_eq!(f("2020-W53-6"), d("2021-01-02"));
        assert_eq!(f("2020-W53-7"), d("2021-01-03"));
        assert_eq!(f("2021-W01-1"), d("2021-01-04"));
    }

    #[test]
    fn from_str() {
        type E = ParseWeekDateError;
        let f = |s: &str| WeekDate::from_str(s);

        assert!(matches!(f("2021-W01-2"), Ok(_)));
        assert!(matches!(f("20021-W01-2"), Err(E::InvalidLength)));
        assert!(matches!(f("2021+W01-2"), Err(E::InvalidFormat)));
        assert!(matches!(f("2021-W01+2"), Err(E::InvalidFormat)));
        assert!(matches!(f("+001-W01-2"), Err(E::ParseYear(_))));
        assert!(matches!(f("2021-W54-2"), Err(E::ParseWeek(_))));
        assert!(matches!(f("2021-W01-8"), Err(E::ParseDayOfWeek(_))));
        assert!(matches!(f("1970-W01-1"), Err(E::InvalidDayOfWeek)));
    }

    #[test]
    fn day_of_week() {
        let d = WeekDate::from_str("2021-W01-2").unwrap();
        assert_eq!(d.day_of_week(), DayOfWeek::from_str("2").unwrap());
    }

    #[test]
    fn week() {
        let d = WeekDate::from_str("2021-W01-2").unwrap();
        assert_eq!(d.week(), Week::from_str("01").unwrap());
    }

    #[test]
    fn to_string() {
        assert_eq!(
            WeekDate::from_str("2021-W01-2").unwrap().to_string(),
            "2021-W01-2".to_string()
        );
    }

    #[test]
    fn year() {
        let d = WeekDate::from_str("2021-W01-2").unwrap();
        assert_eq!(d.year(), WeekYear::from_str("2021").unwrap());
    }

    #[test]
    fn year_week() {
        let d = WeekDate::from_str("2021-W01-2").unwrap();
        assert_eq!(d.year_week(), YearWeek::from_str("2021-W01").unwrap());
    }
}
