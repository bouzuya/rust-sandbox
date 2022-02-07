#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Version(u64);

impl Version {
    pub fn prev(&self) -> Option<Self> {
        if self.0 == std::u64::MIN {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }

    pub fn next(&self) -> Option<Self> {
        if self.0 == std::u64::MAX {
            None
        } else {
            Some(Self(self.0 + 1))
        }
    }
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Version> for u64 {
    fn from(version: Version) -> Self {
        version.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u64_conversion_test() {
        assert_eq!(u64::from(Version::from(u64::MIN)), u64::MIN);
        assert_eq!(u64::from(Version::from(u64::MAX)), u64::MAX);
    }

    #[test]
    fn prev_test() {
        assert!(Version::from(0_u64).prev().is_none());
        assert_eq!(Version::from(1_u64).prev(), Some(Version::from(0_u64)));
    }

    #[test]
    fn next_test() {
        assert_eq!(Version::from(0_u64).next(), Some(Version::from(1_u64)));
        assert!(Version::from(u64::MAX).next().is_none());
    }
}
