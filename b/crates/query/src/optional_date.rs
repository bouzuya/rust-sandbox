use std::str::FromStr;

use limited_date_time::{Date, DateTime, DayOfMonth, Month, Time, Year, YearMonth};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionalDate(Option<Year>, Option<Month>, Option<DayOfMonth>);

impl OptionalDate {
    pub fn from_yyyy(yyyy: Year) -> Self {
        Self(Some(yyyy), None, None)
    }

    pub fn from_yyyymm(yyyy: Year, mm: Month) -> Self {
        Self(Some(yyyy), Some(mm), None)
    }

    pub fn from_yyyymmdd(yyyy: Year, mm: Month, dd: DayOfMonth) -> Self {
        Self(Some(yyyy), Some(mm), Some(dd))
    }

    pub fn year(&self) -> Option<Year> {
        self.0
    }

    pub fn month(&self) -> Option<Month> {
        self.1
    }

    pub fn day_of_month(&self) -> Option<DayOfMonth> {
        self.2
    }

    pub fn naive_date_time_range(&self) -> (DateTime, DateTime) {
        let date_range = match (self.0, self.1, self.2) {
            (None, None, None) => unreachable!(),
            (None, None, Some(_)) => unreachable!(),
            (None, Some(_), None) => unreachable!(),
            (None, Some(_), Some(_)) => unreachable!(),
            (Some(year), None, None) => {
                // TODO: DateRange::from_year(Year)
                let mn = Date::first_date_of_year(year);
                let mx = Date::last_date_of_year(year);
                (mn, mx)
            }
            (Some(_), None, Some(_)) => unreachable!(),
            (Some(year), Some(month), None) => {
                let year_month = YearMonth::new(year, month);
                // TODO: DateRange::from_year_month(YearMonth)
                let mn = Date::first_date_of_month(year_month);
                let mx = Date::last_date_of_month(year_month);
                (mn, mx)
            }
            (Some(year), Some(month), Some(day_of_month)) => {
                // TODO: DateRange::from_date(Date)
                // TODO: unwrap Year -> Month -> DayOfMonth -> Date
                let mn = Date::from_ymd(year, month, day_of_month).unwrap();
                let mx = mn;
                (mn, mx)
            }
        };
        (
            // TODO: DateTimeRange::from_date(Date)
            // TODO: DateTime::first_date_time_of_date(Date)
            // TODO: unwrap Time::min()
            DateTime::from_date_time(date_range.0, Time::from_str("00:00:00").unwrap()),
            // TODO: DateTime::last_date_time_of_date(Date)
            // TODO: unwrap Time::max()
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
        let yyyy = Year::try_from(2021)?;
        let mm = Month::try_from(2)?;
        let dd = DayOfMonth::try_from(3)?;

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
