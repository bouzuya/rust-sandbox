use crate::{week_date::WeekDate, DateRange};

#[derive(Debug, Eq, PartialEq)]
pub enum OutputFormat {
    First,
    Last,
    Range,
}

impl OutputFormat {
    pub fn format(&self, week_date: bool, date_range: &DateRange) -> String {
        if week_date {
            match self {
                OutputFormat::First => WeekDate::from(date_range.first()).to_string(),
                OutputFormat::Last => WeekDate::from(date_range.last()).to_string(),
                OutputFormat::Range => format!(
                    "{}/{}",
                    WeekDate::from(date_range.first()),
                    WeekDate::from(date_range.last())
                ),
            }
        } else {
            match self {
                OutputFormat::First => date_range.first().to_string(),
                OutputFormat::Last => date_range.last().to_string(),
                OutputFormat::Range => format!("{}/{}", date_range.first(), date_range.last()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::InputFormat;

    use super::*;

    #[test]
    fn test() {
        let r = DateRange::parse(&InputFormat::Month, "2021-01").unwrap();
        assert_eq!(
            OutputFormat::First.format(false, &r),
            "2021-01-01".to_string()
        );
        assert_eq!(
            OutputFormat::Last.format(false, &r),
            "2021-01-31".to_string()
        );
        assert_eq!(
            OutputFormat::Range.format(false, &r),
            "2021-01-01/2021-01-31".to_string()
        );

        assert_eq!(
            OutputFormat::First.format(true, &r),
            "2020-W53-5".to_string()
        );
        assert_eq!(
            OutputFormat::Last.format(true, &r),
            "2021-W04-7".to_string()
        );
        assert_eq!(
            OutputFormat::Range.format(true, &r),
            "2020-W53-5/2021-W04-7".to_string()
        );
    }
}
