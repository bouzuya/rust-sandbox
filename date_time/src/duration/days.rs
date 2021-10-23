use crate::Seconds;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Days(u32);

impl From<u32> for Days {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Days> for u32 {
    fn from(value: Days) -> Self {
        value.0
    }
}

// TODO: impl From<Days> for Hours
// TODO: impl From<Days> for Minutes

impl From<Days> for Seconds {
    fn from(value: Days) -> Self {
        Self::from(value.0 as u64 * 86_400_u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_conversion_test() {
        assert_eq!(u32::from(Days::from(0_u32)), 0_u32);
        assert_eq!(u32::from(Days::from(1_u32)), 1_u32);
    }

    #[test]
    fn seconds_conversion_test() {
        assert_eq!(Seconds::from(Days::from(0_u32)), Seconds::from(0_u64));
        assert_eq!(Seconds::from(Days::from(1_u32)), Seconds::from(86_400_u64));
    }
}
