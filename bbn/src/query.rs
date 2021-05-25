use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::all_consuming,
    multi::count,
    IResult,
};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Query(Date);

#[derive(Debug, Eq, PartialEq)]
pub struct Date(Option<String>, Option<String>, Option<String>);

#[derive(Debug, Error)]
pub enum ParseQueryError {
    #[error("parse error")]
    Parse,
}

impl std::fmt::Display for Query {
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

fn yyyymmdd(s: &str) -> IResult<&str, Date> {
    let (s, y) = count(one_of("0123456789"), 4)(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = count(one_of("0123456789"), 2)(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = count(one_of("0123456789"), 2)(s)?;
    Ok((
        s,
        Date(
            Some(y.iter().collect::<String>()),
            Some(m.iter().collect::<String>()),
            Some(d.iter().collect::<String>()),
        ),
    ))
}

fn yyyymm(s: &str) -> IResult<&str, Date> {
    let (s, y) = count(one_of("0123456789"), 4)(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = count(one_of("0123456789"), 2)(s)?;
    Ok((
        s,
        Date(
            Some(y.iter().collect::<String>()),
            Some(m.iter().collect::<String>()),
            None,
        ),
    ))
}

fn yyyy(s: &str) -> IResult<&str, Date> {
    let (s, y) = count(one_of("0123456789"), 4)(s)?;
    Ok((s, Date(Some(y.iter().collect::<String>()), None, None)))
}

fn mmdd(s: &str) -> IResult<&str, Date> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = count(one_of("0123456789"), 2)(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = count(one_of("0123456789"), 2)(s)?;
    Ok((
        s,
        Date(
            None,
            Some(m.iter().collect::<String>()),
            Some(d.iter().collect::<String>()),
        ),
    ))
}

fn mm(s: &str) -> IResult<&str, Date> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = count(one_of("0123456789"), 2)(s)?;
    Ok((s, Date(None, Some(m.iter().collect::<String>()), None)))
}

fn dd(s: &str) -> IResult<&str, Date> {
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = count(one_of("0123456789"), 2)(s)?;
    Ok((s, Date(None, None, Some(d.iter().collect::<String>()))))
}

fn parse(s: &str) -> IResult<&str, Date> {
    let (s, _) = tag("date:")(s)?;
    let (s, date) = all_consuming(alt((yyyymmdd, yyyymm, yyyy, mmdd, mm, dd)))(s)?;
    Ok((s, date))
}

impl std::str::FromStr for Query {
    type Err = ParseQueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, date) = parse(s).map_err(|_| ParseQueryError::Parse)?;
        Ok(Self(date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_convert() {
        let f = |s: &str| assert_eq!(s.parse::<Query>().unwrap().to_string(), s.to_string());
        f("date:2021-02-03");
        f("date:2021-02");
        f("date:2021");
        f("date:--02-03");
        f("date:--02");
        f("date:---03");
    }
}
