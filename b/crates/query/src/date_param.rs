use std::str::FromStr;

use limited_date_time::{DayOfMonth, Month, Year};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::char,
    combinator::{all_consuming, map, map_res},
    sequence::tuple,
    IResult,
};
use thiserror::Error;

use crate::OptionalDate;

// ParseQueryError

#[derive(Debug, Error)]
pub enum ParseDateParamError {
    #[error("parse date param error")]
    Parse,
}

// DateParam

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DateParam {
    Single(OptionalDate),
    Range(DateParamRange),
}

impl std::fmt::Display for DateParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "date:{}",
            match self {
                DateParam::Single(s) => s.to_string(),
                DateParam::Range(r) => r.to_string(),
            }
        )
    }
}

impl FromStr for DateParam {
    type Err = ParseDateParamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        all_consuming(parse)(s)
            .map(|(_, q)| q)
            .map_err(|_| ParseDateParamError::Parse)
    }
}

impl std::convert::TryFrom<&str> for DateParam {
    type Error = ParseDateParamError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        DateParam::from_str(value)
    }
}

// DateRangeDate

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateRangeDate(Year, Month, DayOfMonth);

impl std::fmt::Display for DateRangeDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.0, self.1, self.2)
    }
}

// DateParamRange

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateParamRange(DateRangeDate, DateRangeDate);

impl std::fmt::Display for DateParamRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn date_range_date(s: &str) -> IResult<&str, DateRangeDate> {
    map(
        tuple((year, char('-'), month, char('-'), day_of_month)),
        |(y, _, m, _, d)| DateRangeDate(y, m, d),
    )(s)
}

fn date_range(s: &str) -> IResult<&str, DateParamRange> {
    map(
        tuple((date_range_date, char('/'), date_range_date)),
        |(d1, _, d2)| DateParamRange(d1, d2),
    )(s)
}

fn yyyymmdd(s: &str) -> IResult<&str, OptionalDate> {
    let (s, y) = year(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = month(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = day_of_month(s)?;
    Ok((s, OptionalDate::from_yyyymmdd(y, m, d)))
}

fn yyyymm(s: &str) -> IResult<&str, OptionalDate> {
    let (s, y) = year(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = month(s)?;
    Ok((s, OptionalDate::from_yyyymm(y, m)))
}

fn yyyy(s: &str) -> IResult<&str, OptionalDate> {
    let (s, y) = year(s)?;
    Ok((s, OptionalDate::from_yyyy(y)))
}

fn month(s: &str) -> IResult<&str, Month> {
    map_res(take_while_m_n(2, 2, is_digit), Month::from_str)(s)
}

fn day_of_month(s: &str) -> IResult<&str, DayOfMonth> {
    map_res(take_while_m_n(2, 2, is_digit), DayOfMonth::from_str)(s)
}

fn year(s: &str) -> IResult<&str, Year> {
    map_res(take_while_m_n(4, 4, is_digit), Year::from_str)(s)
}

pub fn parse(s: &str) -> IResult<&str, DateParam> {
    let (s, _) = tag("date:")(s)?;
    let (s, date) = alt((
        map(date_range, DateParam::Range),
        map(alt((yyyymmdd, yyyymm, yyyy)), DateParam::Single),
    ))(s)?;
    Ok((s, date))
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use limited_date_time::DayOfMonth;

    use super::*;

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        let f = |s: &str| -> anyhow::Result<()> {
            assert_eq!(DateParam::from_str(s)?.to_string(), s.to_string());
            Ok(())
        };
        f("date:2021-02-03")?;
        f("date:2021-02")?;
        f("date:2021")?;
        f("date:2021-02-03/2022-03-04")?;
        Ok(())
    }

    #[test]
    fn date_param_single_test() -> anyhow::Result<()> {
        let year = Year::try_from(2021)?;
        let month = Month::try_from(2)?;
        let day_of_month = DayOfMonth::try_from(3)?;
        assert_eq!(
            DateParam::from_str("date:2021-02-03")?,
            DateParam::Single(OptionalDate::from_yyyymmdd(year, month, day_of_month)),
        );
        if let DateParam::Single(date_param_single) = DateParam::from_str("date:2021-02-03")? {
            assert_eq!(date_param_single.year(), Some(year));
            assert_eq!(date_param_single.month(), Some(month));
            assert_eq!(date_param_single.day_of_month(), Some(day_of_month));
        }
        // TODO: "date:2021-02"
        // TODO: "date:2021"
        // TODO: "date:--02-03"
        // TODO: "date:--02"
        // TODO: "date:---03"
        Ok(())
    }
}
