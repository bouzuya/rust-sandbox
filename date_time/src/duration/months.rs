#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Months(u32);

impl From<u32> for Months {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Months> for u32 {
    fn from(value: Months) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_conversion_test() {
        assert_eq!(u32::from(Months::from(0_u32)), 0_u32);
        assert_eq!(u32::from(Months::from(1_u32)), 1_u32);
    }
}
