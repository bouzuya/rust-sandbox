use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take_while1},
        streaming::take_while,
    },
    combinator::{all_consuming, map},
    sequence::delimited,
    IResult,
};
use thiserror::Error;

// ParseTagParamError

#[derive(Debug, Eq, Error, PartialEq)]
#[error("parse tag param error")]
pub struct ParseTagParamError;

// TagParam

#[derive(Debug, Eq, PartialEq)]
pub struct TagParam(String);

impl std::fmt::Display for TagParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.contains(' ') {
            write!(f, "tag:\"{}\"", self.0)
        } else {
            write!(f, "tag:{}", self.0)
        }
    }
}

impl FromStr for TagParam {
    type Err = ParseTagParamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        all_consuming(parse)(s)
            .map(|(_, p)| p)
            .map_err(|_| ParseTagParamError)
    }
}

fn is_ascii_alphanumeric_or_space(c: char) -> bool {
    c == ' ' || c.is_ascii_alphanumeric()
}

pub fn parse(s: &str) -> IResult<&str, TagParam> {
    let (s, _) = tag("tag:")(s)?;
    let (s, t) = map(
        alt((
            delimited(
                tag("\""),
                take_while(is_ascii_alphanumeric_or_space),
                tag("\""),
            ),
            take_while1(char::is_alphanumeric),
        )),
        |s: &str| TagParam(s.to_string()),
    )(s)?;
    Ok((s, t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        let s = "tag:\"abc 123\"";
        assert_eq!(TagParam::from_str(s)?.to_string(), s.to_string());
        let s = "tag:abc";
        assert_eq!(TagParam::from_str(s)?.to_string(), s.to_string());
        Ok(())
    }
}
