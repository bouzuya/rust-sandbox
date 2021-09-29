use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::char,
    combinator::{all_consuming, map, map_res},
    sequence::tuple,
    IResult,
};
use thiserror::Error;

use crate::{Digit2, Digit4};

#[derive(Debug, Eq, PartialEq)]
pub enum Query<'a> {
    Date(Date),
    DateRange(DateRange<'a>),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Date(Option<Digit4>, Option<Digit2>, Option<Digit2>);

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.0, &self.1, &self.2) {
            (None, None, None) => write!(f, ""),
            (None, None, Some(dd)) => write!(f, "---{}", dd),
            (None, Some(mm), None) => write!(f, "--{}", mm),
            (None, Some(mm), Some(dd)) => write!(f, "--{}-{}", mm, dd),
            (Some(yyyy), None, None) => write!(f, "{}", yyyy),
            (Some(_), None, Some(_)) => unreachable!(),
            (Some(yyyy), Some(mm), None) => write!(f, "{}-{}", yyyy, mm),
            (Some(yyyy), Some(mm), Some(dd)) => write!(f, "{}-{}-{}", yyyy, mm, dd),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DateRangeDate<'a>(&'a str, &'a str, &'a str);

impl<'a> std::fmt::Display for DateRangeDate<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.0, self.1, self.2)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DateRange<'a>(DateRangeDate<'a>, DateRangeDate<'a>);

impl<'a> std::fmt::Display for DateRange<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

#[derive(Debug, Error)]
pub enum ParseQueryError {
    #[error("parse error")]
    Parse,
}

impl<'a> std::fmt::Display for Query<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Query::Date(date) => write!(f, "date:{}", date),
            Query::DateRange(date_range) => write!(f, "date:{}", date_range),
        }
    }
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn date_range_date(s: &str) -> IResult<&str, DateRangeDate> {
    map(
        tuple((
            take_while_m_n(4, 4, is_digit),
            char('-'),
            take_while_m_n(2, 2, is_digit),
            char('-'),
            take_while_m_n(2, 2, is_digit),
        )),
        |(y, _, m, _, d)| DateRangeDate(y, m, d),
    )(s)
}

fn date_range(s: &str) -> IResult<&str, DateRange> {
    map(
        tuple((date_range_date, char('/'), date_range_date)),
        |(d1, _, d2)| DateRange(d1, d2),
    )(s)
}

fn yyyymmdd(s: &str) -> IResult<&str, Date> {
    let (s, y) = digit4(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = digit2(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = digit2(s)?;
    Ok((s, Date(Some(y), Some(m), Some(d))))
}

fn yyyymm(s: &str) -> IResult<&str, Date> {
    let (s, y) = digit4(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = digit2(s)?;
    Ok((s, Date(Some(y), Some(m), None)))
}

fn yyyy(s: &str) -> IResult<&str, Date> {
    let (s, y) = digit4(s)?;
    Ok((s, Date(Some(y), None, None)))
}

fn mmdd(s: &str) -> IResult<&str, Date> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = digit2(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = digit2(s)?;
    Ok((s, Date(None, Some(m), Some(d))))
}

fn mm(s: &str) -> IResult<&str, Date> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = digit2(s)?;
    Ok((s, Date(None, Some(m), None)))
}

fn dd(s: &str) -> IResult<&str, Date> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = digit2(s)?;
    Ok((s, Date(None, None, Some(d))))
}

fn digit2(s: &str) -> IResult<&str, Digit2> {
    map_res(take_while_m_n(2, 2, is_digit), Digit2::from_str)(s)
}

fn digit4(s: &str) -> IResult<&str, Digit4> {
    map_res(take_while_m_n(4, 4, is_digit), Digit4::from_str)(s)
}

fn parse(s: &str) -> IResult<&str, Query> {
    if s.is_empty() {
        return Ok((s, Query::Date(Date(None, None, None))));
    }
    let (s, _) = tag("date:")(s)?;
    let (s, date) = all_consuming(alt((
        map(date_range, Query::DateRange),
        map(alt((yyyymmdd, yyyymm, yyyy, mmdd, mm, dd)), |d| {
            Query::Date(d)
        }),
    )))(s)?;
    Ok((s, date))
}

impl<'a> std::convert::TryFrom<&'a str> for Query<'a> {
    type Error = ParseQueryError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        parse(value)
            .map(|(_, q)| Ok(q))
            .map_err(|_| ParseQueryError::Parse)?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        let f = |s: &str| -> anyhow::Result<()> {
            assert_eq!(Query::try_from(s)?.to_string(), s.to_string());
            Ok(())
        };
        f("date:2021-02-03")?;
        f("date:2021-02")?;
        f("date:2021")?;
        f("date:--02-03")?;
        f("date:--02")?;
        f("date:---03")?;
        f("date:2021-02-03/2022-03-04")?;
        Ok(())
    }
}
