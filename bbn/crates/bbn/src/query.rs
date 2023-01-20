use std::ffi::OsStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::char,
    combinator::{all_consuming, map},
    sequence::tuple,
    IResult,
};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum Query<'a> {
    Date(Date<'a>),
    DateRange(DateRange<'a>),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Date<'a>(Option<&'a str>, Option<&'a str>, Option<&'a str>);

impl<'a> Date<'a> {
    fn match_year(&self, year: &OsStr) -> bool {
        self.0.map(|y| OsStr::new(y) == year).unwrap_or(true)
    }

    fn match_month(&self, month: &OsStr) -> bool {
        self.1.map(|m| OsStr::new(m) == month).unwrap_or(true)
    }

    fn match_day(&self, day: &OsStr) -> bool {
        self.2.map(|d| OsStr::new(d) == day).unwrap_or(true)
    }

    fn match_date(&self, date: &str) -> bool {
        let y = match date.get(0..4) {
            None => return false,
            Some(y) => OsStr::new(y),
        };
        let m = match date.get(5..5 + 2) {
            None => return false,
            Some(m) => OsStr::new(m),
        };
        let d = match date.get(8..8 + 2) {
            None => return false,
            Some(d) => OsStr::new(d),
        };
        self.match_year(y) && self.match_month(m) && self.match_day(d)
    }
}

impl<'a> std::fmt::Display for Date<'a> {
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

impl<'a> DateRange<'a> {
    fn match_year(&self, year: &OsStr) -> bool {
        year.to_str()
            .map(|y| (self.0 .0..=self.1 .0).contains(&y))
            .unwrap_or(false)
    }

    fn match_month(&self, month: &OsStr) -> bool {
        if self.0 .0 != self.1 .0 {
            return true;
        }
        month
            .to_str()
            .map(|m| (self.0 .1..=self.1 .1).contains(&m))
            .unwrap_or(false)
    }

    fn match_day(&self, day: &OsStr) -> bool {
        if self.0 .0 != self.1 .0 || self.0 .1 != self.1 .1 {
            return true;
        }
        day.to_str()
            .map(|d| (self.0 .2..=self.1 .2).contains(&d))
            .unwrap_or(false)
    }

    fn match_date(&self, date: &str) -> bool {
        (self.0.to_string().as_str()..=self.1.to_string().as_str()).contains(&date)
    }
}

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

impl<'a> Query<'a> {
    pub fn match_year(&self, year: &OsStr) -> bool {
        match self {
            Query::Date(date) => date.match_year(year),
            Query::DateRange(date_range) => date_range.match_year(year),
        }
    }

    pub fn match_month(&self, month: &OsStr) -> bool {
        match self {
            Query::Date(date) => date.match_month(month),
            Query::DateRange(date_range) => date_range.match_month(month),
        }
    }

    pub fn match_day(&self, day: &OsStr) -> bool {
        match self {
            Query::Date(date) => date.match_day(day),
            Query::DateRange(date_range) => date_range.match_day(day),
        }
    }

    pub fn match_date(&self, date: &str) -> bool {
        match self {
            Query::Date(d) => d.match_date(date),
            Query::DateRange(dr) => dr.match_date(date),
        }
    }
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
    fn str_convert() {
        let f = |s: &str| assert_eq!(Query::try_from(s).unwrap().to_string(), s.to_string());
        f("date:2021-02-03");
        f("date:2021-02");
        f("date:2021");
        f("date:--02-03");
        f("date:--02");
        f("date:---03");
        f("date:2021-02-03/2022-03-04");
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
        assert_eq!(f("date:2021-02-03/2022-03-04", "2020"), false);
        assert_eq!(f("date:2021-02-03/2022-03-04", "2021"), true);
        assert_eq!(f("date:2021-02-03/2022-03-04", "2022"), true);
        assert_eq!(f("date:2021-02-03/2022-03-04", "2023"), false);
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
        assert_eq!(f("date:2021-02-03/2021-03-04", "01"), false);
        assert_eq!(f("date:2021-02-03/2021-03-04", "02"), true);
        assert_eq!(f("date:2021-02-03/2021-03-04", "03"), true);
        assert_eq!(f("date:2021-02-03/2021-03-04", "04"), false);
        assert_eq!(f("date:2021-02-03/2022-03-04", "01"), true);
        assert_eq!(f("date:2021-02-03/2022-03-04", "12"), true);
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
        assert_eq!(f("date:2021-02-03/2021-02-04", "02"), false);
        assert_eq!(f("date:2021-02-03/2021-02-04", "03"), true);
        assert_eq!(f("date:2021-02-03/2021-02-04", "04"), true);
        assert_eq!(f("date:2021-02-03/2021-02-04", "05"), false);
        assert_eq!(f("date:2021-02-03/2021-03-04", "01"), true);
        assert_eq!(f("date:2021-02-03/2021-03-04", "31"), true);
        assert_eq!(f("date:2021-02-03/2022-03-04", "01"), true);
        assert_eq!(f("date:2021-02-03/2022-03-04", "31"), true);
    }

    #[test]
    fn match_date() {
        let f = |s: &str, t: &str| -> bool {
            let q = Query::try_from(s).unwrap();
            q.match_date(t)
        };
        assert_eq!(f("date:2021-02-03", "2021-02-03"), true);
        assert_eq!(f("date:2021-02-03", "2021-02-02"), false);
        assert_eq!(f("date:2021-02", "2021-02-03"), true);
        assert_eq!(f("date:2021-02", "2021-03-01"), false);
        assert_eq!(f("date:2021", "2021-02-03"), true);
        assert_eq!(f("date:2021", "2022-01-01"), false);
        assert_eq!(f("date:--02-03", "2021-02-03"), true);
        assert_eq!(f("date:--02-03", "2020-02-03"), true);
        assert_eq!(f("date:--02-03", "2020-02-04"), false);
        assert_eq!(f("date:2021-02-03/2022-03-04", "2021-02-02"), false);
        assert_eq!(f("date:2021-02-03/2022-03-04", "2021-02-03"), true);
        assert_eq!(f("date:2021-02-03/2022-03-04", "2022-03-04"), true);
        assert_eq!(f("date:2021-02-03/2022-03-04", "2022-03-05"), false);
    }
}
