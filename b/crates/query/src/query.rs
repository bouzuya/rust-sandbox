use std::str::FromStr;

use limited_date_time::DateTime;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list0,
    IResult,
};

use super::date_param::parse as date_param;
use super::tag_param::parse as tag_param;
use crate::QueryParam;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse query error")]
pub struct ParseQueryError;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Query(Vec<QueryParam>);

impl Query {
    pub fn naive_date_time_range(&self) -> (DateTime, DateTime) {
        // TODO: min, max
        let mut min = DateTime::from_str("1970-01-02T00:00:00").unwrap();
        let mut max = DateTime::from_str("9999-12-30T23:59:59").unwrap();
        let empty = (max, min);
        for query_param in self.clone().into_iter() {
            match query_param {
                QueryParam::Date(datea_param) => match datea_param {
                    crate::DateParam::Single(optional_date) => {
                        let (mn, mx) = optional_date.naive_date_time_range();
                        if max < mn || mx < min {
                            return empty;
                        }
                        min = min.max(mn);
                        max = max.min(mx);
                        if min > max {
                            return empty;
                        }
                    }
                    crate::DateParam::Range(_) => todo!(),
                },
                QueryParam::Tag(_) => continue,
            }
        }

        (min, max)
    }
}

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

impl std::str::FromStr for Query {
    type Err = ParseQueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s).map(|(_, q)| q).map_err(|_| ParseQueryError)
    }
}

impl IntoIterator for Query {
    type Item = QueryParam;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{DateParam, TagParam};

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

    #[test]
    fn iterator_test() -> anyhow::Result<()> {
        let q = Query::from_str("date:2021-02-03 tag:abc")?;
        let qps = q.into_iter().collect::<Vec<QueryParam>>();
        assert_eq!(
            qps,
            vec![
                QueryParam::Date(DateParam::from_str("date:2021-02-03")?),
                QueryParam::Tag(TagParam::from_str("tag:abc")?),
            ]
        );
        Ok(())
    }

    #[test]
    fn default_test() -> anyhow::Result<()> {
        assert_eq!(Query::default(), Query::from_str("")?);
        Ok(())
    }

    #[test]
    fn naive_date_time_range() -> anyhow::Result<()> {
        // FIXME:
        assert_eq!(
            Query::from_str("")?.naive_date_time_range(),
            (
                DateTime::from_str("1970-01-02T00:00:00")?,
                DateTime::from_str("9999-12-30T23:59:59")?,
            )
        );
        assert_eq!(
            Query::from_str("date:2021")?.naive_date_time_range(),
            (
                DateTime::from_str("2021-01-01T00:00:00")?,
                DateTime::from_str("2021-12-31T23:59:59")?,
            )
        );
        assert_eq!(
            Query::from_str("date:2021-02")?.naive_date_time_range(),
            (
                DateTime::from_str("2021-02-01T00:00:00")?,
                DateTime::from_str("2021-02-28T23:59:59")?,
            )
        );
        assert_eq!(
            Query::from_str("date:2021-02-03")?.naive_date_time_range(),
            (
                DateTime::from_str("2021-02-03T00:00:00")?,
                DateTime::from_str("2021-02-03T23:59:59")?,
            )
        );
        // TODO: date:2021-02-03/2021-03-04

        Ok(())
    }
}
