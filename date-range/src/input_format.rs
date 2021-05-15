use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InputFormat {
    Date,
    Month,
    Year,
    WeekDate,
    Week,
    WeekYear,
    Quarter,
}

impl InputFormat {
    pub fn detect(s: &str) -> Result<InputFormat, &'static str> {
        let patterns = vec![
            (Self::Date, Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()),
            (Self::Month, Regex::new(r"^\d{4}-\d{2}$").unwrap()),
            (Self::Year, Regex::new(r"^\d{4}$").unwrap()),
            (Self::WeekDate, Regex::new(r"^\d{4}-W\d{2}-\d$").unwrap()),
            (Self::Week, Regex::new(r"^\d{4}-W\d{2}$").unwrap()),
            (Self::WeekYear, Regex::new(r"^\d{4}$").unwrap()), // = year
            (Self::Quarter, Regex::new(r"^\d{4}-Q\d$").unwrap()),
        ];
        patterns
            .into_iter()
            .find_map(|(e, p)| if p.is_match(s) { Some(e) } else { None })
            .ok_or("unknown input format")
    }
}

impl std::str::FromStr for InputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "date" => Ok(Self::Date),
            "month" => Ok(Self::Month),
            "year" => Ok(Self::Year),
            "week-date" => Ok(Self::WeekDate),
            "week" => Ok(Self::Week),
            "week-year" => Ok(Self::WeekYear),
            "quarter" => Ok(Self::Quarter),
            _ => Err("unknown input format"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_test() {
        assert_eq!(InputFormat::detect("2021-01-01"), Ok(InputFormat::Date));
        assert_eq!(InputFormat::detect("2021-01"), Ok(InputFormat::Month));
        assert_eq!(InputFormat::detect("2021"), Ok(InputFormat::Year));
        assert_eq!(InputFormat::detect("2021-W01-1"), Ok(InputFormat::WeekDate));
        assert_eq!(InputFormat::detect("2021-W01"), Ok(InputFormat::Week));
        assert_eq!(InputFormat::detect("2021"), Ok(InputFormat::Year)); // is not WeekYear
        assert_eq!(InputFormat::detect("2021-Q1"), Ok(InputFormat::Quarter));
    }

    #[test]
    fn parse_test() {
        assert_eq!("date".parse(), Ok(InputFormat::Date));
        assert_eq!("month".parse(), Ok(InputFormat::Month));
        assert_eq!("year".parse(), Ok(InputFormat::Year));
        assert_eq!("week-date".parse(), Ok(InputFormat::WeekDate));
        assert_eq!("week".parse(), Ok(InputFormat::Week));
        assert_eq!("week-year".parse(), Ok(InputFormat::WeekYear));
        assert_eq!("quarter".parse(), Ok(InputFormat::Quarter));
    }
}
