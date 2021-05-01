use chrono::prelude::*;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Format {
    Date,
    Month,
    Year,
    Quarter,
    WeekDate,
    Week,
    WeekYear,
}

impl Format {
    pub fn format(&self, dt: &NaiveDate) -> String {
        match self {
            Format::Date => dt.format("%Y-%m-%d").to_string(),
            Format::Month => dt.format("%Y-%m").to_string(),
            Format::Year => dt.format("%Y").to_string(),
            Format::Quarter => format!("{:04}-Q{}", dt.year(), (dt.month() - 1) / 3 + 1),
            Format::WeekDate => dt.format("%G-W%V-%u").to_string(),
            Format::Week => dt.format("%G-W%V").to_string(),
            Format::WeekYear => dt.format("%G").to_string(),
        }
    }
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "date" => Format::Date,
            "month" => Format::Month,
            "year" => Format::Year,
            "quarter" => Format::Quarter,
            "week-date" => Format::WeekDate,
            "week" => Format::Week,
            "week-year" => Format::WeekYear,
            _ => return Err("invalid format"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("date".parse::<Format>(), Ok(Format::Date));
        assert_eq!("month".parse::<Format>(), Ok(Format::Month));
        assert_eq!("year".parse::<Format>(), Ok(Format::Year));
        assert_eq!("quarter".parse::<Format>(), Ok(Format::Quarter));
        assert_eq!("week-date".parse::<Format>(), Ok(Format::WeekDate));
        assert_eq!("week".parse::<Format>(), Ok(Format::Week));
        assert_eq!("week-year".parse::<Format>(), Ok(Format::WeekYear));
    }

    #[test]
    fn test_format_ymd() {
        let dt = NaiveDate::from_ymd(2021, 2, 3);
        assert_eq!(Format::Date.format(&dt), "2021-02-03".to_string());
        assert_eq!(Format::Month.format(&dt), "2021-02".to_string());
        assert_eq!(Format::Year.format(&dt), "2021".to_string());
    }

    #[test]
    fn test_format_quarter() {
        let dt = NaiveDate::from_ymd(2020, 12, 31);
        assert_eq!(Format::Quarter.format(&dt), "2020-Q4".to_string());
        let dt = NaiveDate::from_ymd(2021, 1, 1);
        assert_eq!(Format::Quarter.format(&dt), "2021-Q1".to_string());
        let dt = NaiveDate::from_ymd(2021, 3, 31);
        assert_eq!(Format::Quarter.format(&dt), "2021-Q1".to_string());
        let dt = NaiveDate::from_ymd(2021, 4, 1);
        assert_eq!(Format::Quarter.format(&dt), "2021-Q2".to_string());
        let dt = NaiveDate::from_ymd(2021, 6, 30);
        assert_eq!(Format::Quarter.format(&dt), "2021-Q2".to_string());
        let dt = NaiveDate::from_ymd(2021, 7, 1);
        assert_eq!(Format::Quarter.format(&dt), "2021-Q3".to_string());
        let dt = NaiveDate::from_ymd(2021, 9, 30);
        assert_eq!(Format::Quarter.format(&dt), "2021-Q3".to_string());
        let dt = NaiveDate::from_ymd(2021, 10, 1);
        assert_eq!(Format::Quarter.format(&dt), "2021-Q4".to_string());
        let dt = NaiveDate::from_ymd(2021, 12, 31);
        assert_eq!(Format::Quarter.format(&dt), "2021-Q4".to_string());
        let dt = NaiveDate::from_ymd(2022, 1, 1);
        assert_eq!(Format::Quarter.format(&dt), "2022-Q1".to_string());
    }

    #[test]
    fn test_format_week_date() {
        let dt = NaiveDate::from_ymd(2021, 1, 3);
        assert_eq!(dt, NaiveDate::from_isoywd(2020, 53, Weekday::Sun));
        assert_eq!(Format::WeekDate.format(&dt), "2020-W53-7".to_string());

        let dt = NaiveDate::from_ymd(2021, 1, 4);
        assert_eq!(Format::WeekDate.format(&dt), "2021-W01-1".to_string());
        let dt = NaiveDate::from_ymd(2021, 12, 31);
        assert_eq!(dt, NaiveDate::from_isoywd(2021, 52, Weekday::Fri));
        assert_eq!(Format::WeekDate.format(&dt), "2021-W52-5".to_string());
    }
}
