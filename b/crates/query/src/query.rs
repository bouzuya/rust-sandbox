use std::{cmp, str::FromStr};

use chrono::{NaiveDate, NaiveDateTime};
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
    pub fn naive_date_time_range(&self) -> (NaiveDateTime, NaiveDateTime) {
        let mut min = NaiveDateTime::from_str("0000-01-01T00:00:00").unwrap();
        let mut max = NaiveDateTime::from_str("9999-12-31T23:59:59").unwrap();
        let empty = (max, min);
        for query_param in self.clone().into_iter() {
            match query_param {
                QueryParam::Date(datea_param) => match datea_param {
                    crate::DateParam::Single(date_param_single) => {
                        let date_range = match (
                            date_param_single.year(),
                            date_param_single.month(),
                            date_param_single.day_of_month(),
                        ) {
                            (None, None, None) => continue,
                            (None, None, Some(_)) => unreachable!(),
                            (None, Some(_), None) => unreachable!(),
                            (None, Some(_), Some(_)) => unreachable!(),
                            (Some(yyyy), None, None) => {
                                let year = u16::from(yyyy) as i32;
                                let mn = NaiveDate::from_ymd(year, 1, 1);
                                let mx = NaiveDate::from_ymd(year, 12, 31);
                                (mn, mx)
                            }
                            (Some(_), None, Some(_)) => unreachable!(),
                            (Some(yyyy), Some(mm), None) => {
                                let year = u16::from(yyyy) as i32;
                                let month = u8::from(mm) as u32;
                                let mn = NaiveDate::from_ymd(year, month, 1);
                                let last_day_of_month = NaiveDate::from_ymd(
                                    match month {
                                        12 => year + 1,
                                        _ => year,
                                    },
                                    match month {
                                        12 => 1,
                                        _ => month + 1,
                                    },
                                    1,
                                )
                                .signed_duration_since(mn)
                                .num_days()
                                    as u32;
                                let mx = NaiveDate::from_ymd(year, month, last_day_of_month);
                                (mn, mx)
                            }
                            (Some(yyyy), Some(mm), Some(dd)) => {
                                let year = u16::from(yyyy) as i32;
                                let month = u8::from(mm) as u32;
                                let day_of_month = u8::from(dd) as u32;
                                let mn = NaiveDate::from_ymd(year, month, day_of_month);
                                let mx = mn;
                                (mn, mx)
                            }
                        };
                        let date_time_range = (
                            date_range.0.and_hms(0, 0, 0),
                            date_range.1.and_hms(23, 59, 59),
                        );
                        let (mn, mx) = date_time_range;
                        if max < mn || mx < min {
                            return empty;
                        }
                        min = cmp::max(min, mn);
                        max = cmp::min(max, mx);
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
        assert_eq!(
            Query::from_str("")?.naive_date_time_range(),
            (
                NaiveDateTime::from_str("0000-01-01T00:00:00")?,
                NaiveDateTime::from_str("9999-12-31T23:59:59")?,
            )
        );
        assert_eq!(
            Query::from_str("date:2021")?.naive_date_time_range(),
            (
                NaiveDateTime::from_str("2021-01-01T00:00:00")?,
                NaiveDateTime::from_str("2021-12-31T23:59:59")?,
            )
        );
        assert_eq!(
            Query::from_str("date:2021-02")?.naive_date_time_range(),
            (
                NaiveDateTime::from_str("2021-02-01T00:00:00")?,
                NaiveDateTime::from_str("2021-02-28T23:59:59")?,
            )
        );
        assert_eq!(
            Query::from_str("date:2021-02-03")?.naive_date_time_range(),
            (
                NaiveDateTime::from_str("2021-02-03T00:00:00")?,
                NaiveDateTime::from_str("2021-02-03T23:59:59")?,
            )
        );
        // TODO: date:2021-02-03/2021-03-04

        Ok(())
    }
}
