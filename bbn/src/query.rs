use nom::{
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::all_consuming,
    multi::count,
    IResult,
};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Query((u8, u8));

#[derive(Debug, Error)]
pub enum ParseQueryError {
    #[error("parse error")]
    Parse,
    #[error("invalid number error")]
    InvalidNumber,
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "date:--{:02}-{:02}", self.0 .0, self.0 .1)
    }
}

fn parse(s: &str) -> IResult<&str, (Vec<char>, Vec<char>)> {
    // date:--MM-DD
    let (s, _) = tag("date:")(s)?;
    let (s, _) = char('-')(s)?;
    let (s, _) = char('-')(s)?;
    let (s, m) = count(one_of("0123456789"), 2)(s)?;
    let (s, _) = char('-')(s)?;
    let (s, d) = all_consuming(count(one_of("0123456789"), 2))(s)?;
    Ok((s, (m, d)))
}

impl std::str::FromStr for Query {
    type Err = ParseQueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (m, d)) = parse(s).map_err(|_| ParseQueryError::Parse)?;
        let m = m
            .iter()
            .collect::<String>()
            .parse::<u8>()
            .map_err(|_| ParseQueryError::InvalidNumber)?;
        let d = d
            .iter()
            .collect::<String>()
            .parse::<u8>()
            .map_err(|_| ParseQueryError::InvalidNumber)?;
        Ok(Self((m, d)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_convert() {
        // --MM-DD
        assert_eq!(
            "date:--01-01".parse::<Query>().unwrap().to_string(),
            "date:--01-01".to_string()
        );
        assert_eq!("date:--01-01".parse::<Query>().unwrap(), Query((1, 1)));
    }
}
