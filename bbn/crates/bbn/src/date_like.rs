use std::str::FromStr;

use time::{format_description, Date};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateLike(date_range::date::Date);

impl From<DateLike> for date_range::date::Date {
    fn from(date_like: DateLike) -> Self {
        date_like.0
    }
}

impl FromStr for DateLike {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calendar_date_format = format_description::parse(
            "[year base:calendar repr:full]-[month padding:zero repr:numerical]-[day padding:zero]",
        )?;
        let week_date_format = format_description::parse("[year base:iso_week repr:full]-W[week_number padding:zero repr:iso]-[weekday repr:monday one_indexed:true]")?;
        Date::parse(s, &week_date_format)
            .map_err(|e| anyhow::anyhow!(e))
            .and_then(|d| {
                d.format(&calendar_date_format)
                    .map_err(|e| anyhow::anyhow!(e))
                    .and_then(|s| {
                        date_range::date::Date::from_str(s.as_str()).map_err(|e| anyhow::anyhow!(e))
                    })
                    .map(Self)
            })
            .or_else(|_| {
                Date::parse(s, &calendar_date_format)
                    .map_err(|e| anyhow::anyhow!(e))
                    .and_then(|d| {
                        d.format(&calendar_date_format)
                            .map_err(|e| anyhow::anyhow!(e))
                    })
                    .and_then(|s| {
                        date_range::date::Date::from_str(s.as_str()).map_err(|e| anyhow::anyhow!(e))
                    })
                    .map(Self)
            })
    }
}

#[cfg(test)]
mod tests {
    use time::{
        format_description::{self},
        Date, Month, Weekday,
    };

    use super::*;

    #[test]
    fn test_calendar_date_format() -> anyhow::Result<()> {
        let calendar_date_format = format_description::parse(
            "[year base:calendar repr:full]-[month padding:zero repr:numerical]-[day padding:zero]",
        )?;
        let test_cases = vec![
            (
                Date::from_calendar_date(2023, Month::January, 1)?,
                "2023-01-01",
            ),
            (
                Date::from_calendar_date(2023, Month::December, 31)?,
                "2023-12-31",
            ),
            (
                Date::from_calendar_date(1, Month::January, 1)?,
                "0001-01-01",
            ),
        ];
        for (date, s) in test_cases {
            assert_eq!(Date::parse(s, &calendar_date_format)?, date);
            assert_eq!(date.format(&calendar_date_format)?, s);
        }
        Ok(())
    }

    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        assert_eq!(
            DateLike::from_str("2023-07-05")?,
            DateLike(date_range::date::Date::from_str("2023-07-05")?)
        );
        assert_eq!(
            DateLike::from_str("2023-W27-3")?,
            DateLike(date_range::date::Date::from_str("2023-07-05")?)
        );
        Ok(())
    }

    #[test]
    fn test_week_date_format() -> anyhow::Result<()> {
        let week_date_format = format_description::parse("[year base:iso_week repr:full]-W[week_number padding:zero repr:iso]-[weekday repr:monday one_indexed:true]")?;
        let test_cases = vec![
            (
                Date::from_iso_week_date(2023, 27, Weekday::Wednesday)?,
                "2023-W27-3",
            ),
            (
                Date::from_iso_week_date(2023, 1, Weekday::Monday)?,
                "2023-W01-1",
            ),
            (
                Date::from_iso_week_date(2023, 52, Weekday::Sunday)?,
                "2023-W52-7",
            ),
            (
                Date::from_iso_week_date(2026, 53, Weekday::Thursday)?,
                "2026-W53-4",
            ),
            (
                Date::from_calendar_date(2026, Month::December, 31)?,
                "2026-W53-4",
            ),
            (
                Date::from_iso_week_date(2026, 53, Weekday::Friday)?,
                "2026-W53-5",
            ),
            (
                Date::from_calendar_date(2027, Month::January, 1)?,
                "2026-W53-5",
            ),
            (
                Date::from_iso_week_date(1, 1, Weekday::Monday)?,
                "0001-W01-1",
            ),
        ];
        for (date, s) in test_cases {
            assert_eq!(Date::parse(s, &week_date_format)?, date);
            assert_eq!(date.format(&week_date_format)?, s);
        }
        Ok(())
    }
}
