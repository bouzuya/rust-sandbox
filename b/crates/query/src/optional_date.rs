use chrono::{NaiveDate, NaiveDateTime};

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

    pub fn naive_date_time_range(&self) -> (NaiveDateTime, NaiveDateTime) {
        let date_range = match (self.0, self.1, self.2) {
            (None, None, None) => unreachable!(),
            (None, None, Some(_)) => unreachable!(),
            (None, Some(_), None) => unreachable!(),
            (None, Some(_), Some(_)) => unreachable!(),
            (Some(yyyy), None, None) => {
                let year = u16::from(yyyy) as i32;
                let mn = NaiveDate::from_ymd(year, 1, 1);
                let mx = NaiveDate::from_ymd(year, 12, 31);
                (mn, mx)
            }
            (Some(_), None, Some(_)) => unreachable!(),
            (Some(yyyy), Some(mm), None) => {
                let year = u16::from(yyyy) as i32;
                let month = u8::from(mm) as u32;
                let mn = NaiveDate::from_ymd(year, month, 1);
                let last_day_of_month = NaiveDate::from_ymd(
                    match month {
                        12 => year + 1,
                        _ => year,
                    },
                    match month {
                        12 => 1,
                        _ => month + 1,
                    },
                    1,
                )
                .signed_duration_since(mn)
                .num_days() as u32;
                let mx = NaiveDate::from_ymd(year, month, last_day_of_month);
                (mn, mx)
            }
            (Some(yyyy), Some(mm), Some(dd)) => {
                let year = u16::from(yyyy) as i32;
                let month = u8::from(mm) as u32;
                let day_of_month = u8::from(dd) as u32;
                let mn = NaiveDate::from_ymd(year, month, day_of_month);
                let mx = mn;
                (mn, mx)
            }
        };
        (
            date_range.0.and_hms(0, 0, 0),
            date_range.1.and_hms(23, 59, 59),
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
