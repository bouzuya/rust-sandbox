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

// ParseQueryError

#[derive(Debug, Error)]
pub enum ParseDateParamError {
    #[error("parse date param error")]
    Parse,
}

// DateParam

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DateParam {
    Single(DateParamSingle),
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

// DateParamSingle

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateParamSingle(Option<Digit4>, Option<Digit2>, Option<Digit2>);

impl DateParamSingle {
    pub fn year(&self) -> Option<Digit4> {
        self.0
    }

    pub fn month(&self) -> Option<Digit2> {
        self.1
    }

    pub fn day_of_month(&self) -> Option<Digit2> {
        self.2
    }
}

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

// DateRangeDate

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateRangeDate(Digit4, Digit2, Digit2);

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

pub fn parse(s: &str) -> IResult<&str, DateParam> {
    let (s, _) = tag("date:")(s)?;
    let (s, date) = alt((
        map(date_range, DateParam::Range),
        map(alt((yyyymmdd, yyyymm, yyyy, mmdd, mm, dd)), |d| {
            DateParam::Single(d)
        }),
    ))(s)?;
    Ok((s, date))
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

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
        f("date:--02-03")?;
        f("date:--02")?;
        f("date:---03")?;
        f("date:2021-02-03/2022-03-04")?;
        Ok(())
    }

    #[test]
    fn date_param_single_test() -> anyhow::Result<()> {
        let year = Digit4::try_from(2021)?;
        let month = Digit2::try_from(2)?;
        let day_of_month = Digit2::try_from(3)?;
        assert_eq!(
            DateParam::from_str("date:2021-02-03")?,
            DateParam::Single(DateParamSingle(Some(year), Some(month), Some(day_of_month))),
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
