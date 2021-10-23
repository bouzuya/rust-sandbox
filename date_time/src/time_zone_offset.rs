use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TimeZoneOffset(i16);

#[derive(Debug, Eq, Error, PartialEq)]
pub enum ParseTimeZoneOffsetError {
    #[error("invalid digit")]
    InvalidDigit,
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid length")]
    InvalidLength,
    #[error("out of range")]
    OutOfRange,
}

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TryFromTimeZoneOffsetError {
    #[error("out of range")]
    OutOfRange,
}

impl TimeZoneOffset {
    pub fn from_h(hours: i8) -> Result<TimeZoneOffset, TryFromTimeZoneOffsetError> {
        Self::from_hm(hours, 0)
    }

    pub fn from_hm(hours: i8, minutes: i8) -> Result<TimeZoneOffset, TryFromTimeZoneOffsetError> {
        if hours <= -24 || 24 <= hours {
            return Err(TryFromTimeZoneOffsetError::OutOfRange);
        }

        if minutes <= -60 || 60 <= minutes {
            return Err(TryFromTimeZoneOffsetError::OutOfRange);
        }

        Self::from_offset_in_minutes(hours as i16 * 60 + minutes as i16)
    }

    pub fn from_offset_in_minutes(
        offset_in_minutes: i16,
    ) -> Result<TimeZoneOffset, TryFromTimeZoneOffsetError> {
        if offset_in_minutes <= -1_440_i16 || 1_440_i16 <= offset_in_minutes {
            return Err(TryFromTimeZoneOffsetError::OutOfRange);
        }

        Ok(Self(offset_in_minutes))
    }

    pub fn utc() -> TimeZoneOffset {
        TimeZoneOffset(0)
    }

    pub fn hour(&self) -> i8 {
        (self.0 / 60_i16) as i8
    }

    pub fn minute(&self) -> i8 {
        (self.0 % 60_i16) as i8
    }

    pub fn offset_in_minutes(&self) -> i16 {
        self.0
    }
}

impl std::fmt::Display for TimeZoneOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{:02}:{:02}",
            if self.0.is_negative() { '-' } else { '+' },
            self.0.abs() / 60,
            self.0.abs() % 60
        )
    }
}

impl std::str::FromStr for TimeZoneOffset {
    type Err = ParseTimeZoneOffsetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 6 {
            return Err(Self::Err::InvalidLength);
        }
        let chars = s.chars().collect::<Vec<char>>();
        let signed = match chars[0] {
            '+' => 1,
            '-' => -1,
            _ => return Err(Self::Err::InvalidFormat),
        };

        let mut h = 0_u8;
        for &c in &chars[1..3] {
            let d = match c {
                '0'..='9' => c as u8 - b'0',
                _ => return Err(Self::Err::InvalidDigit),
            };
            h = h * 10 + d;
        }

        if chars[3] != ':' {
            return Err(Self::Err::InvalidFormat);
        }

        let mut m = 0_u8;
        for &c in &chars[4..6] {
            let d = match c {
                '0'..='9' => c as u8 - b'0',
                _ => return Err(Self::Err::InvalidDigit),
            };
            m = m * 10 + d;
        }

        Self::from_offset_in_minutes(signed * (h as i16 * 60 + m as i16))
            .map_err(|_| Self::Err::OutOfRange)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        let f = TimeZoneOffset::from_offset_in_minutes;
        let g = TimeZoneOffset::from_str;
        assert_eq!(f(1_439_i16)?.to_string(), "+23:59");
        assert_eq!(f(540_i16)?.to_string(), "+09:00");
        assert_eq!(f(0_i16)?.to_string(), "+00:00");
        assert_eq!(f(-1_439_i16)?.to_string(), "-23:59");
        assert_eq!(g("+23:59")?.to_string(), "+23:59");
        assert_eq!(g("+09:00")?.to_string(), "+09:00");
        assert_eq!(g("+00:00")?.to_string(), "+00:00");
        assert_eq!(g("-23:59")?.to_string(), "-23:59");
        Ok(())
    }

    #[test]
    fn constructor_test() -> anyhow::Result<()> {
        let offset = TimeZoneOffset::from_h(0)?;
        assert_eq!(offset.to_string(), "+00:00");
        let offset = TimeZoneOffset::from_h(23)?;
        assert_eq!(offset.to_string(), "+23:00");
        let offset = TimeZoneOffset::from_h(-23)?;
        assert_eq!(offset.to_string(), "-23:00");
        let offset = TimeZoneOffset::from_hm(23, 59)?;
        assert_eq!(offset.to_string(), "+23:59");
        let offset = TimeZoneOffset::from_hm(-23, -59)?;
        assert_eq!(offset.to_string(), "-23:59");
        let offset = TimeZoneOffset::from_hm(-23, 59)?;
        assert_eq!(offset.hour(), -22);
        assert_eq!(offset.minute(), -1);
        assert_eq!(offset.to_string(), "-22:01");
        Ok(())
    }

    #[test]
    fn hour_test() -> anyhow::Result<()> {
        assert_eq!(TimeZoneOffset::from_str("+23:59")?.hour(), 23);
        assert_eq!(TimeZoneOffset::from_str("+00:00")?.hour(), 0);
        assert_eq!(TimeZoneOffset::from_str("-23:59")?.hour(), -23);
        Ok(())
    }

    #[test]
    fn minute_test() -> anyhow::Result<()> {
        assert_eq!(TimeZoneOffset::from_str("+23:59")?.minute(), 59);
        assert_eq!(TimeZoneOffset::from_str("+00:00")?.minute(), 0);
        assert_eq!(TimeZoneOffset::from_str("-23:59")?.minute(), -59);
        Ok(())
    }

    #[test]
    fn offset_in_minutes_conversion_test() -> anyhow::Result<()> {
        let f = TimeZoneOffset::from_offset_in_minutes;
        assert!(f(-1_440_i16).is_err());
        assert_eq!(f(-1_439_i16)?.offset_in_minutes(), -1_439_i16);
        assert_eq!(f(0_i16)?.offset_in_minutes(), 0_i16);
        assert_eq!(f(1_439_i16)?.offset_in_minutes(), 1_439_i16);
        assert!(f(1_440_i16).is_err());
        Ok(())
    }
}
