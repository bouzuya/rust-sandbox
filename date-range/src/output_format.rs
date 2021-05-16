use crate::DateRange;

#[derive(Debug, Eq, PartialEq)]
pub enum OutputFormat {
    First,
    Last,
    Range,
}

impl OutputFormat {
    pub fn format(&self, date_range: &DateRange) -> String {
        match self {
            OutputFormat::First => date_range.first().to_string(),
            OutputFormat::Last => date_range.last().to_string(),
            OutputFormat::Range => format!("{}/{}", date_range.first(), date_range.last()),
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
        assert_eq!(OutputFormat::First.format(&r), "2021-01-01".to_string());
        assert_eq!(OutputFormat::Last.format(&r), "2021-01-31".to_string());
        assert_eq!(
            OutputFormat::Range.format(&r),
            "2021-01-01/2021-01-31".to_string()
        );
    }
}
