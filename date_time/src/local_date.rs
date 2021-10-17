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
pub struct LocalDate {
    year: Year,
    month: Month,
    day_of_month: DayOfMonth,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseLocalDateError {
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

#[derive(Debug, Eq, Error, PartialEq)]
#[error("invalid local date error")]
pub struct InvalidLocalDateError;

impl LocalDate {
    pub fn from_ymd(
        year: Year,
        month: Month,
        day_of_month: DayOfMonth,
    ) -> Result<Self, InvalidLocalDateError> {
        let year_month = YearMonth::new(year, month);
        if day_of_month > year_month.last_day_of_month() {
            return Err(InvalidLocalDateError);
        }
        Ok(Self {
            year,
            month,
            day_of_month,
        })
    }

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

    pub fn pred(&self) -> Option<Self> {
        if self.day_of_month() == self.year_month().first_day_of_month() {
            self.year_month().pred().and_then(|last_year_month| {
                // TODO: end_of_month: YearMonth -> LocalDate
                LocalDate::from_ymd(
                    last_year_month.year(),
                    last_year_month.month(),
                    last_year_month.last_day_of_month(),
                )
                .ok()
            })
        } else {
            self.day_of_month().pred().and_then(|next_day_of_month| {
                LocalDate::from_ymd(self.year(), self.month(), next_day_of_month).ok()
            })
        }
    }

    pub fn succ(&self) -> Option<Self> {
        if self.day_of_month() == self.year_month().last_day_of_month() {
            self.year_month().succ().and_then(|next_year_month| {
                // TODO: start_of_month: YearMonth -> LocalDate
                LocalDate::from_ymd(
                    next_year_month.year(),
                    next_year_month.month(),
                    next_year_month.first_day_of_month(),
                )
                .ok()
            })
        } else {
            self.day_of_month().succ().and_then(|next_day_of_month| {
                LocalDate::from_ymd(self.year(), self.month(), next_day_of_month).ok()
            })
        }
    }
}

impl std::fmt::Display for LocalDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day_of_month)
    }
}

impl std::str::FromStr for LocalDate {
    type Err = ParseLocalDateError;

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
        Ok(LocalDate {
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
    fn from_ymd_test() -> anyhow::Result<()> {
        assert_eq!(
            LocalDate::from_ymd(
                Year::from_str("2021")?,
                Month::from_str("02")?,
                DayOfMonth::from_str("03")?
            )?,
            LocalDate::from_str("2021-02-03")?
        );
        assert!(matches!(
            LocalDate::from_ymd(
                Year::from_str("2021")?,
                Month::from_str("02")?,
                DayOfMonth::from_str("31")?
            ),
            Err(InvalidLocalDateError)
        ));
        Ok(())
    }

    #[test]
    fn str_conversion_test() {
        type E = ParseLocalDateError;
        let f = |s: &str| LocalDate::from_str(s);

        assert!(matches!(f("2021-01-02"), Ok(_)));
        assert!(matches!(f("20021-01-02"), Err(E::InvalidLength)));
        assert!(matches!(f("2021+01-02"), Err(E::InvalidFormat)));
        assert!(matches!(f("2021-01+02"), Err(E::InvalidFormat)));
        assert!(matches!(f("+001-01-02"), Err(E::ParseYear(_))));
        assert!(matches!(f("2021-13-02"), Err(E::ParseMonth(_))));
        assert!(matches!(f("2021-01-32"), Err(E::ParseDayOfMonth(_))));
        assert!(matches!(f("2021-02-29"), Err(E::InvalidDayOfMonth)));

        assert_eq!(
            f("2021-01-02").map(|d| d.to_string()),
            Ok("2021-01-02".to_string())
        );
    }

    #[test]
    fn day_of_month_test() -> anyhow::Result<()> {
        let d = LocalDate::from_str("2021-01-02")?;
        assert_eq!(d.day_of_month(), DayOfMonth::from_str("02")?);
        Ok(())
    }

    #[test]
    fn month_test() -> anyhow::Result<()> {
        let d = LocalDate::from_str("2021-01-02")?;
        assert_eq!(d.month(), Month::from_str("01")?);
        Ok(())
    }

    #[test]
    fn year_test() -> anyhow::Result<()> {
        let d = LocalDate::from_str("2021-01-02")?;
        assert_eq!(d.year(), Year::from_str("2021")?);
        Ok(())
    }

    #[test]
    fn year_month_test() -> anyhow::Result<()> {
        let d = LocalDate::from_str("2021-01-02")?;
        assert_eq!(d.year_month(), YearMonth::from_str("2021-01")?);
        Ok(())
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(
            LocalDate::from_str("9999-12-31")?.pred(),
            Some(LocalDate::from_str("9999-12-30")?)
        );
        assert_eq!(
            LocalDate::from_str("1971-01-01")?.pred(),
            Some(LocalDate::from_str("1970-12-31")?)
        );
        assert_eq!(
            LocalDate::from_str("1970-12-01")?.pred(),
            Some(LocalDate::from_str("1970-11-30")?)
        );
        assert_eq!(
            LocalDate::from_str("1970-01-02")?.pred(),
            Some(LocalDate::from_str("1970-01-01")?)
        );
        assert_eq!(LocalDate::from_str("1970-01-01")?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(
            LocalDate::from_str("1970-01-01")?.succ(),
            Some(LocalDate::from_str("1970-01-02")?)
        );
        assert_eq!(
            LocalDate::from_str("9998-12-31")?.succ(),
            Some(LocalDate::from_str("9999-01-01")?)
        );
        assert_eq!(
            LocalDate::from_str("9999-01-31")?.succ(),
            Some(LocalDate::from_str("9999-02-01")?)
        );
        assert_eq!(LocalDate::from_str("9999-12-31")?.succ(), None);
        Ok(())
    }
}
