#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Version(u32);

impl Version {
    pub fn new() -> Self {
        Self(1)
    }

    #[cfg(test)]
    pub fn new_for_testing() -> Self {
        Self(rand::random::<u32>())
    }

    pub fn next(&self) -> Self {
        Self(self.0.checked_add(1).expect("version overflow"))
    }
}

impl std::convert::From<Version> for u32 {
    fn from(value: Version) -> Self {
        value.0
    }
}

impl std::convert::From<u32> for Version {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_from_version_for_u32() {
        assert_eq!(u32::from(Version(std::u32::MIN)), std::u32::MIN);
        assert_eq!(u32::from(Version(1_u32)), 1_u32);
        assert_eq!(u32::from(Version(std::u32::MAX)), std::u32::MAX);
    }

    #[test]
    fn test_impl_from_u32_for_version() {
        assert_eq!(Version::from(std::u32::MIN), Version(std::u32::MIN));
        assert_eq!(Version::from(1_u32), Version(1_u32));
        assert_eq!(Version::from(std::u32::MAX), Version(std::u32::MAX));
    }

    #[test]
    fn test_next() {
        assert_eq!(Version(1).next(), Version(2));
        assert_eq!(Version(2).next(), Version(3));
        assert_eq!(Version(std::u32::MAX - 1).next(), Version(std::u32::MAX));
    }

    #[test]
    #[should_panic(expected = "version overflow")]
    fn test_next_overflow() {
        Version(std::u32::MAX).next();
    }

    #[test]
    fn test_new() {
        assert_eq!(Version::new(), Version(1));
    }
}
