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
}
