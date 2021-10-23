use crate::{DayOfYear, ParseDayOfYearError, ParseYearError, Year};

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OrdinalDate {
    year: Year,
    day_of_year: DayOfYear,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseOrdinalDateError {
    #[error("invalid day of year")]
    InvalidDayOfYear,
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid length")]
    InvalidLength,
    #[error("parse day of year")]
    ParseDayOfYear(ParseDayOfYearError),
    #[error("parse year")]
    ParseYear(ParseYearError),
}

#[derive(Debug, Eq, Error, PartialEq)]
#[error("invalid ordinal date error")]
pub struct InvalidOrdinalDateError;

impl OrdinalDate {
    pub fn first_date_of_year(year: Year) -> Self {
        Self {
            year,
            day_of_year: year.first_day_of_year(),
        }
    }

    pub fn last_date_of_year(year: Year) -> Self {
        Self {
            year,
            day_of_year: year.last_day_of_year(),
        }
    }

    pub fn new(year: Year, day_of_year: DayOfYear) -> Result<Self, InvalidOrdinalDateError> {
        if day_of_year > year.last_day_of_year() {
            return Err(InvalidOrdinalDateError);
        }
        Ok(Self { year, day_of_year })
    }

    pub fn day_of_year(&self) -> DayOfYear {
        self.day_of_year
    }

    pub fn year(&self) -> Year {
        self.year
    }

    pub fn pred(&self) -> Option<Self> {
        if self.day_of_year() == self.year().first_day_of_year() {
            self.year().pred().map(Self::last_date_of_year)
        } else {
            self.day_of_year()
                .pred()
                .and_then(|last_day_of_year| OrdinalDate::new(self.year(), last_day_of_year).ok())
        }
    }

    pub fn succ(&self) -> Option<Self> {
        if self.day_of_year() == self.year().last_day_of_year() {
            self.year().succ().map(Self::first_date_of_year)
        } else {
            self.day_of_year()
                .succ()
                .and_then(|next_day_of_month| OrdinalDate::new(self.year(), next_day_of_month).ok())
        }
    }
}

impl std::fmt::Display for OrdinalDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.year, self.day_of_year)
    }
}

impl std::str::FromStr for OrdinalDate {
    type Err = ParseOrdinalDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            return Err(Self::Err::InvalidLength);
        }
        let year = Year::from_str(&s[0..4]).map_err(Self::Err::ParseYear)?;
        if s.as_bytes().get(4) != Some(&b'-') {
            return Err(Self::Err::InvalidFormat);
        }
        let day_of_year = DayOfYear::from_str(&s[5..8]).map_err(Self::Err::ParseDayOfYear)?;
        Self::new(year, day_of_year).map_err(|_| Self::Err::InvalidDayOfYear)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn first_date_of_year_test() -> anyhow::Result<()> {
        let year = Year::from_str("2021")?;
        assert_eq!(
            OrdinalDate::first_date_of_year(year),
            OrdinalDate::from_str("2021-001")?,
        );
        Ok(())
    }

    #[test]
    fn last_date_of_year_test() -> anyhow::Result<()> {
        let year = Year::from_str("2021")?;
        assert_eq!(
            OrdinalDate::last_date_of_year(year),
            OrdinalDate::from_str("2021-365")?,
        );
        let year = Year::from_str("2000")?;
        assert_eq!(
            OrdinalDate::last_date_of_year(year),
            OrdinalDate::from_str("2000-366")?,
        );
        Ok(())
    }

    #[test]
    fn new_test() -> anyhow::Result<()> {
        assert_eq!(
            OrdinalDate::new(Year::from_str("2021")?, DayOfYear::from_str("001")?)?,
            OrdinalDate::from_str("2021-001")?
        );
        assert!(matches!(
            OrdinalDate::new(Year::from_str("2021")?, DayOfYear::from_str("366")?),
            Err(InvalidOrdinalDateError)
        ));
        Ok(())
    }

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        type E = ParseOrdinalDateError;
        let f = |s: &str| OrdinalDate::from_str(s);

        assert!(matches!(f("2021-001"), Ok(_)));
        assert!(matches!(f("20021-001"), Err(E::InvalidLength)));
        assert!(matches!(f("2021+001"), Err(E::InvalidFormat)));
        assert!(matches!(f("+001-001"), Err(E::ParseYear(_))));
        assert!(matches!(f("2021-+01"), Err(E::ParseDayOfYear(_))));
        assert!(matches!(f("2021-366"), Err(E::InvalidDayOfYear)));

        assert_eq!(f("2021-001")?.to_string(), "2021-001");
        Ok(())
    }

    #[test]
    fn day_of_year_test() -> anyhow::Result<()> {
        let d = OrdinalDate::from_str("2021-001")?;
        assert_eq!(d.day_of_year(), DayOfYear::from_str("001")?);
        Ok(())
    }

    #[test]
    fn year_test() -> anyhow::Result<()> {
        let d = OrdinalDate::from_str("2021-001")?;
        assert_eq!(d.year(), Year::from_str("2021")?);
        Ok(())
    }

    #[test]
    fn pred_test() -> anyhow::Result<()> {
        assert_eq!(
            OrdinalDate::from_str("9999-365")?.pred(),
            Some(OrdinalDate::from_str("9999-364")?)
        );
        assert_eq!(
            OrdinalDate::from_str("1971-001")?.pred(),
            Some(OrdinalDate::from_str("1970-365")?)
        );
        assert_eq!(
            OrdinalDate::from_str("1970-002")?.pred(),
            Some(OrdinalDate::from_str("1970-001")?)
        );
        assert_eq!(OrdinalDate::from_str("1970-001")?.pred(), None);
        Ok(())
    }

    #[test]
    fn succ_test() -> anyhow::Result<()> {
        assert_eq!(
            OrdinalDate::from_str("1970-001")?.succ(),
            Some(OrdinalDate::from_str("1970-002")?)
        );
        assert_eq!(
            OrdinalDate::from_str("9998-365")?.succ(),
            Some(OrdinalDate::from_str("9999-001")?)
        );
        assert_eq!(OrdinalDate::from_str("9999-365")?.succ(), None);
        Ok(())
    }
}
