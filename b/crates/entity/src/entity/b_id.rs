use chrono::{DateTime, NaiveDateTime, Utc};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse bid error")]
pub struct ParseBIdError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BId(i64);

impl std::fmt::Display for BId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_naive_date_time().format("%Y%m%dT%H%M%SZ"))
    }
}

impl FromStr for BId {
    type Err = ParseBIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('Z') {
            let ndt =
                NaiveDateTime::parse_from_str(s, "%Y%m%dT%H%M%SZ").map_err(|_| ParseBIdError)?;
            Ok(BId(ndt.timestamp()))
        } else {
            let dt =
                DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%:z").map_err(|_| ParseBIdError)?;
            Ok(BId(dt.timestamp()))
        }
    }
}

impl BId {
    pub fn now() -> Self {
        let utc = Utc::now();
        Self(utc.timestamp())
    }

    pub fn from_timestamp(i: i64) -> Self {
        Self::from_timestamp_opt(i).expect("invalid timestamp")
    }

    pub fn from_timestamp_opt(i: i64) -> Option<Self> {
        if (0..=253402300799).contains(&i) {
            Some(Self(i))
        } else {
            None
        }
    }

    pub fn to_timestamp(&self) -> i64 {
        self.0
    }

    fn to_naive_date_time(self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use std::str::FromStr;

    #[test]
    fn timestamp_convert_test() {
        let now = BId::now();
        let now1 = Utc::now().timestamp();
        assert_eq!(now.to_timestamp(), now1);
        assert_eq!(BId::from_timestamp(now1).to_timestamp(), now1);

        let min_timestamp = 0;
        let max_timestamp = 253402300799;
        assert_eq!(
            min_timestamp,
            NaiveDateTime::from_str("1970-01-01T00:00:00")
                .unwrap()
                .timestamp()
        );
        assert_eq!(
            max_timestamp,
            NaiveDateTime::from_str("9999-12-31T23:59:59")
                .unwrap()
                .timestamp()
        );

        assert!(!BId::from_timestamp_opt(min_timestamp - 1).is_some());
        assert!(BId::from_timestamp_opt(min_timestamp).is_some());
        assert!(BId::from_timestamp_opt(max_timestamp).is_some());
        assert!(!BId::from_timestamp_opt(max_timestamp + 1).is_some());
    }

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        let s = "20210203T161718Z";
        assert_eq!(BId::from_str(s)?.to_string(), s.to_string());
        assert_eq!(
            BId::from_str("2021-02-03T16:17:18+09:00")?.to_string(),
            "20210203T071718Z".to_string()
        );
        Ok(())
    }
}
