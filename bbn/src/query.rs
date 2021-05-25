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
}
