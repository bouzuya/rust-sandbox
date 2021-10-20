#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Seconds(u64);

impl From<u64> for Seconds {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Seconds> for u64 {
    fn from(value: Seconds) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u64_conversion_test() {
        assert_eq!(u64::from(Seconds::from(0_u64)), 0_u64);
        assert_eq!(u64::from(Seconds::from(1_u64)), 1_u64);
    }
}
