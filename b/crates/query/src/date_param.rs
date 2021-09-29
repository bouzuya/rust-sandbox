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
pub enum DateParam {
    Single(DateParamSingle),
    Range(DateParamRange),
}

#[derive(Debug, Eq, PartialEq)]
pub struct DateParamSingle(Option<Digit4>, Option<Digit2>, Option<Digit2>);

impl std::fmt::Display for DateParamSingle {
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
pub struct DateRangeDate(Digit4, Digit2, Digit2);

impl std::fmt::Display for DateRangeDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.0, self.1, self.2)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DateParamRange(DateRangeDate, DateRangeDate);

impl std::fmt::Display for DateParamRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

#[derive(Debug, Error)]
pub enum ParseQueryError {
    #[error("parse error")]
    Parse,
}

impl std::fmt::Display for DateParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateParam::Single(date) => write!(f, "date:{}", date),
            DateParam::Range(date_range) => write!(f, "date:{}", date_range),
        }
    }
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn date_range_date(s: &str) -> IResult<&str, DateRangeDate> {
    map(
        tuple((digit4, char('-'), digit2, char('-'), digit2)),
        |(y, _, m, _, d)| DateRangeDate(y, m, d),
    )(s)
}

fn date_range(s: &str) -> IResult<&str, DateParamRange> {
    map(
        tuple((date_range_date, char('/'), date_range_date)),
        |(d1, _, d2)| DateParamRange(d1, d2),
    )(s)
}

fn yyyymmdd(s: &str) -> IResult<&str, DateParamSingle> {
    let (s, y) = digit4(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = digit2(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = digit2(s)?;
    Ok((s, DateParamSingle(Some(y), Some(m), Some(d))))
}

fn yyyymm(s: &str) -> IResult<&str, DateParamSingle> {
    let (s, y) = digit4(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = digit2(s)?;
    Ok((s, DateParamSingle(Some(y), Some(m), None)))
}

fn yyyy(s: &str) -> IResult<&str, DateParamSingle> {
    let (s, y) = digit4(s)?;
    Ok((s, DateParamSingle(Some(y), None, None)))
}

fn mmdd(s: &str) -> IResult<&str, DateParamSingle> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = digit2(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = digit2(s)?;
    Ok((s, DateParamSingle(None, Some(m), Some(d))))
}

fn mm(s: &str) -> IResult<&str, DateParamSingle> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = digit2(s)?;
    Ok((s, DateParamSingle(None, Some(m), None)))
}

fn dd(s: &str) -> IResult<&str, DateParamSingle> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = digit2(s)?;
    Ok((s, DateParamSingle(None, None, Some(d))))
}

fn digit2(s: &str) -> IResult<&str, Digit2> {
    map_res(take_while_m_n(2, 2, is_digit), Digit2::from_str)(s)
}

fn digit4(s: &str) -> IResult<&str, Digit4> {
    map_res(take_while_m_n(4, 4, is_digit), Digit4::from_str)(s)
}

fn parse(s: &str) -> IResult<&str, DateParam> {
    if s.is_empty() {
        return Ok((s, DateParam::Single(DateParamSingle(None, None, None))));
    }
    let (s, _) = tag("date:")(s)?;
    let (s, date) = all_consuming(alt((
        map(date_range, DateParam::Range),
        map(alt((yyyymmdd, yyyymm, yyyy, mmdd, mm, dd)), |d| {
            DateParam::Single(d)
        }),
    )))(s)?;
    Ok((s, date))
}

impl std::convert::TryFrom<&str> for DateParam {
    type Error = ParseQueryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
            assert_eq!(DateParam::try_from(s)?.to_string(), s.to_string());
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
