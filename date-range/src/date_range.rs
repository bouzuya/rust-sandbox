use crate::{DateString, InputFormat};
use chrono::{Datelike, NaiveDate, Weekday};

#[derive(Debug, Eq, PartialEq)]
pub struct DateRange {
    first: DateString,
    last: DateString,
}

impl DateRange {
    fn new(first: NaiveDate, last: NaiveDate) -> DateRange {
        DateRange {
            first: DateString(first.format("%Y-%m-%d").to_string()),
            last: DateString(last.format("%Y-%m-%d").to_string()),
        }
    }

    pub fn parse(fmt: &InputFormat, s: &str) -> Result<DateRange, &'static str> {
        // TODO: error message
        match fmt {
            InputFormat::Date => {
                let d = NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();
                Ok(Self::new(d, d))
            }
            InputFormat::Month => {
                let first = NaiveDate::parse_from_str(&format!("{}-01", s), "%Y-%m-%d").unwrap();
                let last = if first.month() == 12 {
                    NaiveDate::from_ymd(first.year() + 1, 1, 1)
                } else {
                    NaiveDate::from_ymd(first.year(), first.month() + 1, 1)
                }
                .pred();
                Ok(Self::new(first, last))
            }
            InputFormat::Year => {
                let d = NaiveDate::parse_from_str(&format!("{}-01-01", s), "%Y-%m-%d").unwrap();
                let first = NaiveDate::from_ymd(d.year(), 1, 1);
                let last = NaiveDate::from_ymd(d.year(), 12, 31);
                Ok(Self::new(first, last))
            }
            InputFormat::WeekDate => {
                let d = NaiveDate::parse_from_str(s, "%G-W%V-%u").unwrap();
                Ok(Self::new(d, d))
            }
            InputFormat::Week => {
                let w = NaiveDate::parse_from_str(&format!("{}-1", s), "%G-W%V-%u")
                    .unwrap()
                    .iso_week();
                let first = NaiveDate::from_isoywd(w.year(), w.week(), Weekday::Mon);
                let last = NaiveDate::from_isoywd(w.year(), w.week(), Weekday::Sun);
                Ok(Self::new(first, last))
            }
            InputFormat::WeekYear => {
                let first =
                    NaiveDate::parse_from_str(&format!("{}-W01-1", s), "%G-W%V-%u").unwrap();
                let last =
                    NaiveDate::from_isoywd(first.iso_week().year() + 1, 1, Weekday::Mon).pred();
                Ok(Self::new(first, last))
            }
            InputFormat::Quarter => {
                let y: u32 = s[0..4].parse().unwrap();
                let q = match s[6..7].parse().unwrap() {
                    1 => (format!("{}-01-01", y), format!("{}-03-31", y)),
                    2 => (format!("{}-04-01", y), format!("{}-06-30", y)),
                    3 => (format!("{}-07-01", y), format!("{}-09-30", y)),
                    4 => (format!("{}-10-01", y), format!("{}-12-31", y)),
                    _ => todo!(),
                };
                Ok(Self::new(
                    NaiveDate::parse_from_str(&q.0, "%Y-%m-%d").unwrap(),
                    NaiveDate::parse_from_str(&q.1, "%Y-%m-%d").unwrap(),
                ))
            }
        }
    }

    pub fn first(&self) -> DateString {
        self.first.clone()
    }

    pub fn last(&self) -> DateString {
        self.last.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use InputFormat::*;

    #[test]
    fn test() {
        let parse = DateRange::parse;
        let f = |first: &str, last: &str| DateRange {
            first: DateString(first.to_string()),
            last: DateString(last.to_string()),
        };
        assert_eq!(
            parse(&Date, "2021-02-03"),
            Ok(f("2021-02-03", "2021-02-03")),
        );
        // assert_eq!(parse(&Date, "2021-02-30").is_err(), true);

        assert_eq!(parse(&Month, "2021-02"), Ok(f("2021-02-01", "2021-02-28")),);
        // assert_eq!(parse(&Month, "2021-13").is_err(), true);

        assert_eq!(
            parse(&Quarter, "2021-Q1"),
            Ok(f("2021-01-01", "2021-03-31")),
        );
        // assert_eq!(parse(&Quarter, "2021-Q5").is_err(), true);

        assert_eq!(parse(&Year, "2021"), Ok(f("2021-01-01", "2021-12-31")),);
        // assert_eq!(parse(&Year, "0000").is_err(), true);

        assert_eq!(
            parse(&WeekDate, "2021-W01-1"),
            Ok(f("2021-01-04", "2021-01-04")),
        );
        // assert_eq!(parse(&WeekDate, "2021-W01-8").is_err(), true);

        assert_eq!(parse(&Week, "2020-W53"), Ok(f("2020-12-28", "2021-01-03")),);
        assert_eq!(parse(&Week, "2021-W01"), Ok(f("2021-01-04", "2021-01-10")),);
        // assert_eq!(parse(&Week, "2020-W54").is_err(), true);
    }
}
