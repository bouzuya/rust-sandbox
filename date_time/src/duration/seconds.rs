#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Seconds(u64);

impl Seconds {
    pub fn from_seconds(seconds: u64) -> Self {
        Self(seconds)
    }

    pub fn to_seconds(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u64_conversion_test() {
        assert_eq!(Seconds::from_seconds(0_u64).to_seconds(), 0_u64);
        assert_eq!(Seconds::from_seconds(1_u64).to_seconds(), 1_u64);
    }
}
