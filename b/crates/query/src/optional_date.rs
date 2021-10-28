use std::{convert::TryFrom, str::FromStr};

use limited_date_time::{Date, DateTime, DayOfMonth, Month, OrdinalDate, Time, Year, YearMonth};

use crate::{Digit2, Digit4};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionalDate(Option<Digit4>, Option<Digit2>, Option<Digit2>);

impl OptionalDate {
    pub fn from_yyyy(yyyy: Digit4) -> Self {
        Self(Some(yyyy), None, None)
    }

    pub fn from_yyyymm(yyyy: Digit4, mm: Digit2) -> Self {
        Self(Some(yyyy), Some(mm), None)
    }

    pub fn from_yyyymmdd(yyyy: Digit4, mm: Digit2, dd: Digit2) -> Self {
        Self(Some(yyyy), Some(mm), Some(dd))
    }

    pub fn year(&self) -> Option<Digit4> {
        self.0
    }

    pub fn month(&self) -> Option<Digit2> {
        self.1
    }

    pub fn day_of_month(&self) -> Option<Digit2> {
        self.2
    }

    pub fn naive_date_time_range(&self) -> (DateTime, DateTime) {
        let date_range = match (self.0, self.1, self.2) {
            (None, None, None) => unreachable!(),
            (None, None, Some(_)) => unreachable!(),
            (None, Some(_), None) => unreachable!(),
            (None, Some(_), Some(_)) => unreachable!(),
            (Some(yyyy), None, None) => {
                // TODO: unwrap
                let year = Year::try_from(u16::from(yyyy)).unwrap();
                // TODO: Date::first_date_of_year(year)
                // TODO: OrdinalDate::first_date_of_year(year)
                let first_ordinal_date_of_year =
                    OrdinalDate::new(year, year.first_day_of_year()).unwrap();
                let mn = Date::from(first_ordinal_date_of_year);
                // TODO: Date::last_date_of_year(year)
                // TODO: OrdinalDate::last_date_of_year(year)
                let last_ordinal_date_of_year =
                    OrdinalDate::new(year, year.last_day_of_year()).unwrap();
                let mx = Date::from(last_ordinal_date_of_year);
                (mn, mx)
            }
            (Some(_), None, Some(_)) => unreachable!(),
            (Some(yyyy), Some(mm), None) => {
                // TODO: unwrap
                let year = Year::try_from(u16::from(yyyy)).unwrap();
                // TODO: unwrap
                let month = Month::try_from(u8::from(mm)).unwrap();
                let year_month = YearMonth::new(year, month);
                let mn = Date::first_date_of_month(year_month);
                let mx = Date::last_date_of_month(year_month);
                (mn, mx)
            }
            (Some(yyyy), Some(mm), Some(dd)) => {
                // TODO: unwrap
                let year = Year::try_from(u16::from(yyyy)).unwrap();
                // TODO: unwrap
                let month = Month::try_from(u8::from(mm)).unwrap();
                // TODO: unwrap
                let day_of_month = DayOfMonth::try_from(u8::from(dd)).unwrap();
                // TODO: unwrap
                let mn = Date::from_ymd(year, month, day_of_month).unwrap();
                let mx = mn;
                (mn, mx)
            }
        };
        (
            // TODO: Time::min()
            // TODO: unwrap
            DateTime::from_date_time(date_range.0, Time::from_str("00:00:00").unwrap()),
            // TODO: Time::max()
            // TODO: unwrap
            DateTime::from_date_time(date_range.1, Time::from_str("23:59:59").unwrap()),
        )
    }
}

impl std::fmt::Display for OptionalDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.0, &self.1, &self.2) {
            (None, _, _) => unreachable!(),
            (Some(yyyy), None, None) => write!(f, "{}", yyyy),
            (Some(_), None, Some(_)) => unreachable!(),
            (Some(yyyy), Some(mm), None) => write!(f, "{}-{}", yyyy, mm),
            (Some(yyyy), Some(mm), Some(dd)) => write!(f, "{}-{}-{}", yyyy, mm, dd),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let yyyy = Digit4::try_from(2021)?;
        let mm = Digit2::try_from(2)?;
        let dd = Digit2::try_from(3)?;

        {
            let d = OptionalDate::from_yyyy(yyyy);
            assert_eq!(d.year(), Some(yyyy));
            assert_eq!(d.month(), None);
            assert_eq!(d.day_of_month(), None);
            assert_eq!(d.to_string(), "2021");
        }

        {
            let d = OptionalDate::from_yyyymm(yyyy, mm);
            assert_eq!(d.year(), Some(yyyy));
            assert_eq!(d.month(), Some(mm));
            assert_eq!(d.day_of_month(), None);
            assert_eq!(d.to_string(), "2021-02");
        }

        {
            let d = OptionalDate::from_yyyymmdd(yyyy, mm, dd);
            assert_eq!(d.year(), Some(yyyy));
            assert_eq!(d.month(), Some(mm));
            assert_eq!(d.day_of_month(), Some(dd));
            assert_eq!(d.to_string(), "2021-02-03");
        }

        Ok(())
    }
}
