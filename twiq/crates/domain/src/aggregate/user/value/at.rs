use std::{fmt::Display, str::FromStr};

use time::{format_description::well_known::Iso8601, OffsetDateTime};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[error("error")]
pub struct Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct At(OffsetDateTime);

impl At {
    pub fn now() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}

impl Display for At {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .format(&Iso8601::DEFAULT)
                .expect("invalid offset date time")
        )
    }
}

impl FromStr for At {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OffsetDateTime::parse(s, &Iso8601::DEFAULT)
            .map(Self)
            .map_err(|_| Error)
    }
}

impl TryFrom<String> for At {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        let s = "2021-02-03T04:05:06.000000000Z";
        let at = At::from_str(s)?;
        assert_eq!(at.to_string(), s);
        let at = At::try_from(s.to_owned())?;
        assert_eq!(at.to_string(), s);
        Ok(())
    }
}
