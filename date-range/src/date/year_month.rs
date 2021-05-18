use std::convert::TryFrom;
use thiserror::Error;

use super::{DayOfMonth, Month, ParseMonthError, ParseYearError, Year};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct YearMonth {
    year: Year,
    month: Month,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseYearMonthError {
    #[error("invalid length")]
    InvalidLength,
    #[error("invalid format")]
    InvalidFormat,
    #[error("parse year")]
    ParseYear(ParseYearError),
    #[error("parse month")]
    ParseMonth(ParseMonthError),
}

impl YearMonth {
    pub fn new(year: Year, month: Month) -> Self {
        Self { year, month }
    }

    pub fn last_day_of_month(&self) -> DayOfMonth {
        let m: u8 = self.month.into();
        let d: u8 = match m {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.year.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!(),
        };
        DayOfMonth::try_from(d).expect("invalid day of month")
    }

    pub fn month(&self) -> Month {
        self.month
    }

    pub fn year(&self) -> Year {
        self.year
    }
}

impl std::fmt::Display for YearMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.year, self.month)
    }
}

impl std::str::FromStr for YearMonth {
    type Err = ParseYearMonthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            return Err(Self::Err::InvalidLength);
        }
        if s.as_bytes().get(4) != Some(&b'-') {
            return Err(Self::Err::InvalidFormat);
        }
        let year = match Year::from_str(&s[0..4]) {
            Ok(y) => y,
            Err(e) => return Err(Self::Err::ParseYear(e)),
        };
        let month = match Month::from_str(&s[5..7]) {
            Ok(m) => m,
            Err(e) => return Err(Self::Err::ParseMonth(e)),
        };
        Ok(Self { year, month })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn month() {
        assert_eq!(
            YearMonth::from_str("2021-01").unwrap().month(),
            Month::from_str("01").unwrap()
        );
    }

    #[test]
    fn new() {
        assert_eq!(
            YearMonth::new(
                Year::from_str("2021").unwrap(),
                Month::from_str("01").unwrap()
            ),
            YearMonth::from_str("2021-01").unwrap()
        );
    }

    #[test]
    fn year() {
        assert_eq!(
            YearMonth::from_str("2021-01").unwrap().year(),
            Year::from_str("2021").unwrap()
        );
    }

    #[test]
    fn str_convert() {
        type E = ParseYearMonthError;
        let f = |s| YearMonth::from_str(s);
        assert_eq!(
            f("2000-01").map(|ym| ym.to_string()),
            Ok("2000-01".to_string())
        );
        assert!(matches!(f("20000-01"), Err(E::InvalidLength)));
        assert!(matches!(f("2000+01"), Err(E::InvalidFormat)));
        assert!(matches!(f("+000-01"), Err(E::ParseYear(_))));
        assert!(matches!(f("2000-13"), Err(E::ParseMonth(_))));
    }

    #[test]
    fn last_day_of_month() {
        let f = |s: &str| -> DayOfMonth { YearMonth::from_str(s).unwrap().last_day_of_month() };
        let d = |d: u8| -> DayOfMonth { DayOfMonth::try_from(d).unwrap() };
        assert_eq!(f("1999-01"), d(31));
        assert_eq!(f("1999-02"), d(28));
        assert_eq!(f("1999-03"), d(31));
        assert_eq!(f("1999-04"), d(30));
        assert_eq!(f("1999-05"), d(31));
        assert_eq!(f("1999-06"), d(30));
        assert_eq!(f("1999-07"), d(31));
        assert_eq!(f("1999-08"), d(31));
        assert_eq!(f("1999-09"), d(30));
        assert_eq!(f("1999-10"), d(31));
        assert_eq!(f("1999-11"), d(30));
        assert_eq!(f("1999-12"), d(31));
        assert_eq!(f("2000-02"), d(29));
    }
}
