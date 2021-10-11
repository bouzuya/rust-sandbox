mod day_of_month;
mod month;
mod year;
mod year_month;

pub use self::day_of_month::{DayOfMonth, ParseDayOfMonthError};
pub use self::month::{Month, ParseMonthError};
pub use self::year::{ParseYearError, Year};
pub use self::year_month::{ParseYearMonthError, YearMonth};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Date {
    year: Year,
    month: Month,
    day_of_month: DayOfMonth,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseDateError {
    #[error("invalid day of month")]
    InvalidDayOfMonth,
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid length")]
    InvalidLength,
    #[error("parse day of month")]
    ParseDayOfMonth(ParseDayOfMonthError),
    #[error("parse month")]
    ParseMonth(ParseMonthError),
    #[error("parse year")]
    ParseYear(ParseYearError),
}

impl Date {
    pub fn day_of_month(&self) -> DayOfMonth {
        self.day_of_month
    }

    pub fn month(&self) -> Month {
        self.month
    }

    pub fn year(&self) -> Year {
        self.year
    }

    pub fn year_month(&self) -> YearMonth {
        YearMonth::new(self.year, self.month)
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day_of_month)
    }
}

impl std::str::FromStr for Date {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(Self::Err::InvalidLength);
        }
        let year_month = match YearMonth::from_str(&s[0..7]) {
            Ok(ym) => ym,
            Err(e) => match e {
                ParseYearMonthError::InvalidLength => unreachable!(),
                ParseYearMonthError::InvalidFormat => return Err(Self::Err::InvalidFormat),
                ParseYearMonthError::ParseYear(e) => return Err(Self::Err::ParseYear(e)),
                ParseYearMonthError::ParseMonth(e) => return Err(Self::Err::ParseMonth(e)),
            },
        };
        if s.as_bytes().get(7) != Some(&b'-') {
            return Err(Self::Err::InvalidFormat);
        }
        let day_of_month = match DayOfMonth::from_str(&s[8..10]) {
            Ok(d) => d,
            Err(e) => return Err(Self::Err::ParseDayOfMonth(e)),
        };
        if day_of_month > year_month.last_day_of_month() {
            return Err(Self::Err::InvalidDayOfMonth);
        }
        Ok(Date {
            year: year_month.year(),
            month: year_month.month(),
            day_of_month,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn from_str() {
        type E = ParseDateError;
        let f = |s: &str| Date::from_str(s);

        assert!(matches!(f("2021-01-02"), Ok(_)));
        assert!(matches!(f("20021-01-02"), Err(E::InvalidLength)));
        assert!(matches!(f("2021+01-02"), Err(E::InvalidFormat)));
        assert!(matches!(f("2021-01+02"), Err(E::InvalidFormat)));
        assert!(matches!(f("+001-01-02"), Err(E::ParseYear(_))));
        assert!(matches!(f("2021-13-02"), Err(E::ParseMonth(_))));
        assert!(matches!(f("2021-01-32"), Err(E::ParseDayOfMonth(_))));
        assert!(matches!(f("2021-02-29"), Err(E::InvalidDayOfMonth)));
    }

    #[test]
    fn day_of_month() {
        let d = Date::from_str("2021-01-02").unwrap();
        assert_eq!(d.day_of_month(), DayOfMonth::from_str("02").unwrap());
    }

    #[test]
    fn month() {
        let d = Date::from_str("2021-01-02").unwrap();
        assert_eq!(d.month(), Month::from_str("01").unwrap());
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Date::from_str("2021-01-02").unwrap().to_string(),
            "2021-01-02".to_string()
        );
    }

    #[test]
    fn year() {
        let d = Date::from_str("2021-01-02").unwrap();
        assert_eq!(d.year(), Year::from_str("2021").unwrap());
    }

    #[test]
    fn year_month() {
        let d = Date::from_str("2021-01-02").unwrap();
        assert_eq!(d.year_month(), YearMonth::from_str("2021-01").unwrap());
    }
}
