use std::num::TryFromIntError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("try from error")]
    TryFrom(#[from] TryFromIntError),
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Version(u32);

impl From<Version> for i64 {
    fn from(value: Version) -> Self {
        i64::from(u32::from(value))
    }
}

impl From<Version> for u32 {
    fn from(value: Version) -> Self {
        value.0
    }
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<i64> for Version {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Version::from(u32::try_from(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i64_conversion_test() -> anyhow::Result<()> {
        assert!(Version::try_from(-1_i64).is_err());
        assert_eq!(Version::try_from(0_i64)?, Version::from(0_u32));
        assert_eq!(i64::from(Version::try_from(0_i64)?), 0_i64);
        Ok(())
    }

    #[test]
    fn u32_conversion_test() {
        assert_eq!(u32::from(Version::from(0_u32)), 0_u32);
    }

    #[test]
    fn default_test() {
        assert_eq!(Version::from(0_u32), Version::default());
    }
}
