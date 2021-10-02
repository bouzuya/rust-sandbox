use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list0,
    IResult,
};

use super::date_param::parse as date_param;
use super::tag_param::parse as tag_param;
use crate::{DateParam, TagParam};

use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse query error")]
pub struct ParseQueryError;

#[derive(Debug, Eq, PartialEq)]
pub enum QueryParam {
    Date(DateParam),
    Tag(TagParam),
}

impl std::fmt::Display for QueryParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                QueryParam::Date(d) => d.to_string(),
                QueryParam::Tag(t) => t.to_string(),
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Query(Vec<QueryParam>);

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

fn parse(s: &str) -> IResult<&str, Query> {
    map(
        all_consuming(separated_list0(
            tag(" "),
            alt((
                map(date_param, QueryParam::Date),
                map(tag_param, QueryParam::Tag),
            )),
        )),
        Query,
    )(s)
}

impl std::str::FromStr for Query {
    type Err = ParseQueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s).map(|(_, q)| q).map_err(|_| ParseQueryError)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        assert_eq!(
            Query::from_str("date:2021-02-03 tag:abc")?.to_string(),
            "date:2021-02-03 tag:abc".to_string()
        );
        assert_eq!(
            Query::from_str("date:2021")?.to_string(),
            "date:2021".to_string()
        );
        Ok(())
    }
}
