use date_range::{date::Date, week_date::WeekDate};
use nom::{
    bytes::complete::take_while_m_n, character::complete::char, combinator::all_consuming,
    sequence::separated_pair, IResult,
};
use thiserror::Error;

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn year_month(s: &str) -> IResult<&str, (&str, &str)> {
    all_consuming(separated_pair(
        take_while_m_n(4, 4, is_digit),
        char('-'),
        take_while_m_n(2, 2, is_digit),
    ))(s)
}

fn week_range_try_from_month(m: &str) -> Option<(&str, &str)> {
    Some(match m {
        "01" => ("01", "04"),
        "02" => ("05", "08"),
        "03" => ("09", "12"),
        // Q1 => 13
        "04" => ("14", "17"),
        "05" => ("18", "21"),
        "06" => ("22", "25"),
        // Q2 => 26
        "07" => ("27", "30"),
        "08" => ("31", "34"),
        "09" => ("35", "38"),
        // Q3 => 39
        "10" => ("40", "44"),
        "11" => ("45", "48"),
        "12" => ("49", "52"),
        // Q4 => 53 (or 52)
        _ => return None,
    })
}

#[derive(Debug, Error)]
pub enum BbnDateRangeError {
    #[error("invalid input")]
    Parse,
    #[error("invalid month {0}")]
    InvalidMonth(String),
    #[error("invalid week date{0}")]
    InvalidWeekDate(String),
}

pub fn bbn_date_range(month: String, week_date: bool) -> Result<(), BbnDateRangeError> {
    let (_, (y, m)) = year_month(month.as_str()).map_err(|_| BbnDateRangeError::Parse)?;
    let (f, l) = week_range_try_from_month(m)
        .ok_or_else(|| BbnDateRangeError::InvalidMonth(m.to_string()))?;
    let first_week_date_string = format!("{}-W{}-1", y, f);
    let first_week_date: WeekDate = first_week_date_string
        .parse()
        .map_err(|_| BbnDateRangeError::InvalidWeekDate(first_week_date_string))?;
    let last_week_date_string = format!("{}-W{}-7", y, l);
    let last_week_date: WeekDate = last_week_date_string
        .parse()
        .map_err(|_| BbnDateRangeError::InvalidWeekDate(last_week_date_string))?;
    let message = if week_date {
        format!("{}/{}", first_week_date, last_week_date)
    } else {
        format!(
            "{}/{}",
            Date::from(first_week_date),
            Date::from(last_week_date)
        )
    };
    println!("{}", message);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year_month_test() {
        // check format only
        assert_eq!(year_month("2021-00"), Ok(("", ("2021", "00"))));
        assert_eq!(year_month("2021-01"), Ok(("", ("2021", "01"))));
        assert_eq!(year_month("2021-12"), Ok(("", ("2021", "12"))));
        assert_eq!(year_month("2021-13"), Ok(("", ("2021", "13"))));
        assert_eq!(year_month("2021-0a").is_err(), true);
    }

    #[test]
    fn week_range_try_from_month_test() {
        let f = week_range_try_from_month;
        assert_eq!(f("00").is_none(), true);
        assert_eq!(f("01"), Some(("01", "04")));
        assert_eq!(f("04"), Some(("14", "17")));
        assert_eq!(f("12"), Some(("49", "52")));
        assert_eq!(f("13").is_none(), true);
    }
}
