use std::str::FromStr;

use chrono::NaiveDateTime;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("timestamp error")]
pub struct TimestampError;

pub(crate) fn date_time_string_from_timestamp(timestamp: i64) -> Result<String, TimestampError> {
    let naive_date_time = NaiveDateTime::from_timestamp(timestamp, 0);
    Ok(format!("{:?}", naive_date_time))
}

pub(crate) fn timestamp_from_date_time_string(
    date_time_string: &str,
) -> Result<i64, TimestampError> {
    Ok(NaiveDateTime::from_str(date_time_string)
        .map_err(|_| TimestampError)?
        .timestamp())
}

pub(crate) fn year_to_days_from_ce(y: i64) -> i64 {
    y * 365 + y / 4 - y / 100 + y / 400
}

#[cfg(test)]
mod tests {
    use crate::Instant;

    use super::*;

    #[test]
    fn date_time_string_from_timestamp_test() -> anyhow::Result<()> {
        let f = date_time_string_from_timestamp;
        let min_timestamp = u64::from(Instant::min()) as i64;
        let max_timestamp = u64::from(Instant::max()) as i64;
        assert_eq!(f(min_timestamp - 1)?, "1969-12-31T23:59:59");
        assert_eq!(f(min_timestamp)?, "1970-01-01T00:00:00");
        assert_eq!(f(max_timestamp)?, "9999-12-31T23:59:59");
        assert_eq!(f(max_timestamp + 1)?, "+10000-01-01T00:00:00");
        Ok(())
    }

    #[test]
    fn timestamp_from_date_time_string_test() -> anyhow::Result<()> {
        let f = timestamp_from_date_time_string;
        let min_timestamp = u64::from(Instant::min()) as i64;
        let max_timestamp = u64::from(Instant::max()) as i64;
        assert_eq!(f("1969-12-31T23:59:59")?, min_timestamp - 1);
        assert_eq!(f("1970-01-01T00:00:00")?, min_timestamp);
        assert_eq!(f("9999-12-31T23:59:59")?, max_timestamp);
        assert_eq!(f("+10000-01-01T00:00:00")?, max_timestamp + 1);
        Ok(())
    }

    #[test]
    fn year_to_days_from_ce_test() -> anyhow::Result<()> {
        let f = year_to_days_from_ce;
        let g =
            |y| chrono::Datelike::num_days_from_ce(&chrono::NaiveDate::from_ymd(y as i32, 1, 1));
        assert_eq!(f(0), 0); // 0000-12-31 ... 0 d
        assert_eq!(g(1), 1); // 0001-01-01 ... 1 d
        assert_eq!(f(1), 365); // 0001-12-31 ... 365 d
        assert_eq!(g(2), 366); // 0002-01-01 ... 366 d
        assert_eq!(f(2), 730); // 0002-12-31 ... 730 d
        assert_eq!(g(3), 731); // 0003-01-01 ... 731 d
        assert_eq!(f(1969), 719162); // 1969-12-31 ... 719162 d
        assert_eq!(g(1970), 719163); // 1970-01-01 ... 719163 d
        for y in 1..=9999 + 1 {
            assert_eq!(f(y - 1) + 1, i64::from(g(y)));
        }
        Ok(())
    }
}
