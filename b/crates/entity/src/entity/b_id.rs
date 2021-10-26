use limited_date_time::{Instant, OffsetDateTime, TimeZoneOffset};
use std::{convert::TryFrom, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse bid error")]
pub struct ParseBIdError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BId(i64);

impl std::fmt::Display for BId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let offset_date_time =
            OffsetDateTime::from_instant(self.to_instant(), TimeZoneOffset::utc())
                .map_err(|_| std::fmt::Error)?;
        write!(
            f,
            "{}",
            b_id_string_from_date_time_string(offset_date_time.to_string())
        )
    }
}

impl FromStr for BId {
    type Err = ParseBIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('Z') {
            let t = date_time_string_from_b_id_string(s).ok_or(ParseBIdError)?;
            let offset_date_time =
                OffsetDateTime::from_str(t.as_str()).map_err(|_| ParseBIdError)?;
            let instant = offset_date_time.instant();
            Ok(BId(timestamp_from_instant(instant)))
        } else {
            let offset_date_time = OffsetDateTime::from_str(s).map_err(|_| ParseBIdError)?;
            let instant = offset_date_time.instant();
            Ok(BId(timestamp_from_instant(instant)))
        }
    }
}

impl BId {
    pub fn now() -> Self {
        let instant = Instant::now();
        Self(timestamp_from_instant(instant))
    }

    pub fn from_timestamp(i: i64) -> Self {
        Self::from_timestamp_opt(i).expect("invalid timestamp")
    }

    pub fn from_timestamp_opt(i: i64) -> Option<Self> {
        // 253402300799 = 9999-12-31T23:59:59Z
        if (0..=253402300799).contains(&i) {
            Some(Self(i))
        } else {
            None
        }
    }

    pub fn to_timestamp(&self) -> i64 {
        self.0
    }

    fn to_instant(self) -> Instant {
        Instant::try_from(self.0).expect("BId i64 invalid")
    }
}

// YYYYMMDDTHHMMSSZ -> YYYY-MM-DDTHH:MM:SSZ
fn date_time_string_from_b_id_string(s: &str) -> Option<String> {
    if s.len() != 16 {
        return None;
    }
    let chars = s.chars().collect::<Vec<char>>();
    let mut t = vec![];
    t.extend(&chars[0..4]);
    t.push('-');
    t.extend(&chars[4..6]);
    t.push('-');
    t.extend(&chars[6..11]);
    t.push(':');
    t.extend(&chars[11..13]);
    t.push(':');
    t.extend(&chars[13..16]);
    Some(t.into_iter().collect::<String>())
}

fn b_id_string_from_date_time_string(s: String) -> String {
    s.chars()
        .filter(|c| c.is_ascii_digit() || c == &'T' || c == &'Z')
        .collect::<String>()
}

// TODO: impl From<Instant> from i64
fn timestamp_from_instant(instant: Instant) -> i64 {
    u64::from(instant) as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use limited_date_time::{Instant, OffsetDateTime};
    use std::{convert::TryFrom, str::FromStr};

    #[test]
    fn instant_to_timestamp_test() -> anyhow::Result<()> {
        let instant = Instant::try_from(0_u64)?;
        assert_eq!(timestamp_from_instant(instant), 0_i64);
        let instant = Instant::try_from(253402300799_u64)?;
        assert_eq!(timestamp_from_instant(instant), 253402300799_i64);
        Ok(())
    }

    #[test]
    fn b_id_string_from_date_time_string_test() {
        assert_eq!(
            b_id_string_from_date_time_string("2021-02-03T04:05:06Z".to_string()),
            "20210203T040506Z"
        );
    }

    #[test]
    fn date_time_string_from_b_id_string_test() {
        assert_eq!(
            date_time_string_from_b_id_string("20210203T040506Z"),
            Some("2021-02-03T04:05:06Z".to_string())
        );
    }

    #[test]
    fn timestamp_convert_test() -> anyhow::Result<()> {
        let now = BId::now();
        let now1 = timestamp_from_instant(Instant::now());
        assert_eq!(now.to_timestamp(), now1);
        assert_eq!(BId::from_timestamp(now1).to_timestamp(), now1);

        let min_timestamp = 0;
        let max_timestamp = 253402300799;
        assert_eq!(
            min_timestamp,
            timestamp_from_instant(OffsetDateTime::from_str("1970-01-01T00:00:00Z")?.instant()),
        );
        assert_eq!(
            max_timestamp,
            timestamp_from_instant(OffsetDateTime::from_str("9999-12-31T23:59:59Z")?.instant()),
        );

        assert!(!BId::from_timestamp_opt(min_timestamp - 1).is_some());
        assert!(BId::from_timestamp_opt(min_timestamp).is_some());
        assert!(BId::from_timestamp_opt(max_timestamp).is_some());
        assert!(!BId::from_timestamp_opt(max_timestamp + 1).is_some());
        Ok(())
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
