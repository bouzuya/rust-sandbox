use std::convert::TryFrom;
use thiserror::Error;

use crate::{Days, Months};

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

    pub fn days(&self) -> Days {
        Days::from(u32::from(u8::from(self.last_day_of_month())))
    }

    pub fn first_day_of_month(&self) -> DayOfMonth {
        DayOfMonth::try_from(1).expect("invalid day of month")
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

    pub fn pred(&self) -> Option<Self> {
        match self.month().pred() {
            Some(last_month) => Some(Self::new(self.year(), last_month)),
            None => self
                .year()
                .pred()
                .map(|last_year| Self::new(last_year, Month::december())),
        }
    }

    pub fn succ(&self) -> Option<Self> {
        match self.month().succ() {
            Some(next_month) => Some(Self::new(self.year(), next_month)),
            None => self
                .year()
                .succ()
                .map(|next_year| Self::new(next_year, Month::january())),
        }
    }
}

impl std::fmt::Display for YearMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.year, self.month)
    }
}

impl std::ops::Add<Months> for YearMonth {
    type Output = YearMonth;

    fn add(self, rhs: Months) -> Self::Output {
        let y = u32::from(u16::from(self.year()));
        let m = u32::from(u8::from(self.month()));
        let ry = u32::from(rhs) / 12;
        let rm = u32::from(rhs) % 12;
        let ny = y + ry + (m + rm) / 12;
        let nm = (m + rm) % 12;
        // TODO: unwrap
        let y = u16::try_from(ny).unwrap();
        let m = u8::try_from(nm).unwrap();
        // TODO: TryFrom<u32> for Year
        // TODO: TryFrom<u32> for Month
        Self::new(Year::try_from(y).unwrap(), Month::try_from(m).unwrap())
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
    fn month_test() -> anyhow::Result<()> {
        let year = Year::from_str("2021")?;
        let month = Month::from_str("01")?;
        let year_month = YearMonth::new(year, month);
        assert_eq!(year_month.month(), month);
        Ok(())
    }

    #[test]
    fn new_test() -> anyhow::Result<()> {
        let year = Year::from_str("2021")?;
        let month = Month::from_str("01")?;
        let year_month = YearMonth::new(year, month);
        assert_eq!(year_month, YearMonth::from_str("2021-01")?);
        Ok(())
    }

    #[test]
    fn year_test() -> anyhow::Result<()> {
        let year = Year::from_str("2021")?;
        let month = Month::from_str("01")?;
        let year_month = YearMonth::new(year, month);
        assert_eq!(year_month.year(), year);
        Ok(())
    }

    #[test]
    fn str_conversion_test() {
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
    fn first_day_of_month_test() -> anyhow::Result<()> {
        assert_eq!(
            YearMonth::from_str("2021-02")?.first_day_of_month(),
            DayOfMonth::from_str("01")?
        );
        Ok(())
    }

    #[test]
    fn last_day_of_month_test() -> anyhow::Result<()> {
        let f = |s: &str| -> anyhow::Result<DayOfMonth> {
            Ok(YearMonth::from_str(s)?.last_day_of_month())
        };
        let d = |d: &str| -> anyhow::Result<DayOfMonth> { Ok(DayOfMonth::from_str(d)?) };
        assert_eq!(f("1999-01")?, d("31")?);
        assert_eq!(f("1999-02")?, d("28")?);
        assert_eq!(f("1999-03")?, d("31")?);
        assert_eq!(f("1999-04")?, d("30")?);
        assert_eq!(f("1999-05")?, d("31")?);
        assert_eq!(f("1999-06")?, d("30")?);
        assert_eq!(f("1999-07")?, d("31")?);
        assert_eq!(f("1999-08")?, d("31")?);
        assert_eq!(f("1999-09")?, d("30")?);
        assert_eq!(f("1999-10")?, d("31")?);
        assert_eq!(f("1999-11")?, d("30")?);
        assert_eq!(f("1999-12")?, d("31")?);
        assert_eq!(f("2000-02")?, d("29")?);
        Ok(())
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(
            YearMonth::from_str("9999-12")?.pred(),
            Some(YearMonth::from_str("9999-11")?)
        );
        assert_eq!(
            YearMonth::from_str("1971-01")?.pred(),
            Some(YearMonth::from_str("1970-12")?)
        );
        assert_eq!(
            YearMonth::from_str("1970-02")?.pred(),
            Some(YearMonth::from_str("1970-01")?)
        );
        assert_eq!(YearMonth::from_str("1970-01")?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(
            YearMonth::from_str("1970-01")?.succ(),
            Some(YearMonth::from_str("1970-02")?)
        );
        assert_eq!(
            YearMonth::from_str("1970-12")?.succ(),
            Some(YearMonth::from_str("1971-01")?)
        );
        assert_eq!(
            YearMonth::from_str("9999-11")?.succ(),
            Some(YearMonth::from_str("9999-12")?)
        );
        assert_eq!(YearMonth::from_str("9999-12")?.succ(), None);
        Ok(())
    }

    #[test]
    fn days_test() -> anyhow::Result<()> {
        assert_eq!(YearMonth::from_str("2000-01")?.days(), Days::from(31));
        assert_eq!(YearMonth::from_str("2000-02")?.days(), Days::from(29));
        assert_eq!(YearMonth::from_str("2001-02")?.days(), Days::from(28));
        assert_eq!(YearMonth::from_str("2000-04")?.days(), Days::from(30));
        Ok(())
    }

    #[test]
    fn add_months_test() -> anyhow::Result<()> {
        assert_eq!(
            YearMonth::from_str("2000-01")? + Months::from(1),
            YearMonth::from_str("2000-02")?
        );
        assert_eq!(
            YearMonth::from_str("2000-01")? + Months::from(2),
            YearMonth::from_str("2000-03")?
        );
        assert_eq!(
            YearMonth::from_str("2000-01")? + Months::from(12),
            YearMonth::from_str("2001-01")?
        );
        assert_eq!(
            YearMonth::from_str("2000-01")? + Months::from(13),
            YearMonth::from_str("2001-02")?
        );
        // should panic
        // YearMonth::from_str("9999-12")? + Months::from(1),
        Ok(())
    }
}
