use std::ffi::OsStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::char,
    combinator::all_consuming,
    IResult,
};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Query<'a>(Date<'a>);

#[derive(Debug, Eq, PartialEq)]
pub struct Date<'a>(Option<&'a str>, Option<&'a str>, Option<&'a str>);

#[derive(Debug, Error)]
pub enum ParseQueryError {
    #[error("parse error")]
    Parse,
}

impl<'a> Query<'a> {
    pub fn match_year(&self, year: &OsStr) -> bool {
        self.0 .0.map(|y| OsStr::new(y) == year).unwrap_or(true)
    }

    pub fn match_month(&self, month: &OsStr) -> bool {
        self.0 .1.map(|m| OsStr::new(m) == month).unwrap_or(true)
    }

    pub fn match_day(&self, day: &OsStr) -> bool {
        self.0 .2.map(|d| OsStr::new(d) == day).unwrap_or(true)
    }
}

impl<'a> std::fmt::Display for Query<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = &self.0;
        match (&date.0, &date.1, &date.2) {
            (None, None, None) => write!(f, ""),
            (None, None, Some(dd)) => write!(f, "date:---{}", dd),
            (None, Some(mm), None) => write!(f, "date:--{}", mm),
            (None, Some(mm), Some(dd)) => write!(f, "date:--{}-{}", mm, dd),
            (Some(yyyy), None, None) => write!(f, "date:{}", yyyy),
            (Some(_), None, Some(_)) => unreachable!(),
            (Some(yyyy), Some(mm), None) => write!(f, "date:{}-{}", yyyy, mm),
            (Some(yyyy), Some(mm), Some(dd)) => write!(f, "date:{}-{}-{}", yyyy, mm, dd),
        }
    }
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn yyyymmdd(s: &str) -> IResult<&str, Date> {
    let (s, y) = take_while_m_n(4, 4, is_digit)(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = take_while_m_n(2, 2, is_digit)(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = take_while_m_n(2, 2, is_digit)(s)?;
    Ok((s, Date(Some(y), Some(m), Some(d))))
}

fn yyyymm(s: &str) -> IResult<&str, Date> {
    let (s, y) = take_while_m_n(4, 4, is_digit)(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = take_while_m_n(2, 2, is_digit)(s)?;
    Ok((s, Date(Some(y), Some(m), None)))
}

fn yyyy(s: &str) -> IResult<&str, Date> {
    let (s, y) = take_while_m_n(4, 4, is_digit)(s)?;
    Ok((s, Date(Some(y), None, None)))
}

fn mmdd(s: &str) -> IResult<&str, Date> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = take_while_m_n(2, 2, is_digit)(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = take_while_m_n(2, 2, is_digit)(s)?;
    Ok((s, Date(None, Some(m), Some(d))))
}

fn mm(s: &str) -> IResult<&str, Date> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = take_while_m_n(2, 2, is_digit)(s)?;
    Ok((s, Date(None, Some(m), None)))
}

fn dd(s: &str) -> IResult<&str, Date> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = take_while_m_n(2, 2, is_digit)(s)?;
    Ok((s, Date(None, None, Some(d))))
}

fn parse(s: &str) -> IResult<&str, Date> {
    let (s, _) = tag("date:")(s)?;
    let (s, date) = all_consuming(alt((yyyymmdd, yyyymm, yyyy, mmdd, mm, dd)))(s)?;
    Ok((s, date))
}

impl<'a> std::convert::TryFrom<&'a str> for Query<'a> {
    type Error = ParseQueryError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (_, date) = parse(value).map_err(|_| ParseQueryError::Parse)?;
        Ok(Self(date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn str_convert() {
        let f = |s: &str| assert_eq!(Query::try_from(s).unwrap().to_string(), s.to_string());
        f("date:2021-02-03");
        f("date:2021-02");
        f("date:2021");
        f("date:--02-03");
        f("date:--02");
        f("date:---03");
    }

    #[test]
    fn match_year() {
        let f = |s: &str, t: &str| -> bool {
            let q = Query::try_from(s).unwrap();
            q.match_year(&OsStr::new(t))
        };
        assert_eq!(f("date:2021-02-03", "2021"), true);
        assert_eq!(f("date:2021-02-03", "2020"), false);
        assert_eq!(f("date:2021-02", "2021"), true);
        assert_eq!(f("date:2021", "2021"), true);
        assert_eq!(f("date:--02-03", "2021"), true);
        assert_eq!(f("date:--02-03", "2020"), true);
    }

    #[test]
    fn match_month() {
        let f = |s: &str, t: &str| -> bool {
            let q = Query::try_from(s).unwrap();
            q.match_month(&OsStr::new(t))
        };
        assert_eq!(f("date:2021-02-03", "02"), true);
        assert_eq!(f("date:2021-02-03", "01"), false);
        assert_eq!(f("date:2021-02", "02"), true);
        assert_eq!(f("date:2021", "02"), true);
        assert_eq!(f("date:2021", "01"), true);
        assert_eq!(f("date:---03", "02"), true);
        assert_eq!(f("date:---03", "01"), true);
    }

    #[test]
    fn match_day() {
        let f = |s: &str, t: &str| -> bool {
            let q = Query::try_from(s).unwrap();
            q.match_day(&OsStr::new(t))
        };
        assert_eq!(f("date:2021-02-03", "03"), true);
        assert_eq!(f("date:2021-02-03", "02"), false);
        assert_eq!(f("date:---03", "03"), true);
        assert_eq!(f("date:---03", "02"), false);
        assert_eq!(f("date:2021-02", "03"), true);
    }
}
