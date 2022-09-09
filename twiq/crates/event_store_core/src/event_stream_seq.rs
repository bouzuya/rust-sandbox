#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("out of range error")]
    OutOfRange,
    #[error("overflow error")]
    Overflow,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventStreamSeq(u32);

impl EventStreamSeq {
    pub fn next(self) -> Result<Self, Error> {
        self.0.checked_add(1).map(Self).ok_or(Error::Overflow)
    }
}

impl From<EventStreamSeq> for i64 {
    fn from(version: EventStreamSeq) -> Self {
        i64::from(version.0)
    }
}

impl From<EventStreamSeq> for u32 {
    fn from(value: EventStreamSeq) -> Self {
        value.0
    }
}

impl From<u32> for EventStreamSeq {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<i64> for EventStreamSeq {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value = u32::try_from(value).map_err(|_| Error::OutOfRange)?;
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_conversion_test() {
        assert_eq!(u32::from(EventStreamSeq::from(u32::MIN)), u32::MIN);
        assert_eq!(u32::from(EventStreamSeq::from(u32::MAX)), u32::MAX);
    }

    #[test]
    fn i64_conversion_test() {
        assert!(EventStreamSeq::try_from(-1_i64).is_err());
        assert!(EventStreamSeq::try_from(0_i64).is_ok());
        assert!(EventStreamSeq::try_from(1_i64).is_ok());
        assert!(EventStreamSeq::try_from(i64::from(u32::MAX)).is_ok());
        assert!(EventStreamSeq::try_from(i64::from(u32::MAX) + 1).is_err());
    }

    #[test]
    fn next_test() -> anyhow::Result<()> {
        assert_eq!(
            EventStreamSeq::from(1_u32).next()?,
            EventStreamSeq::from(2_u32)
        );
        assert!(EventStreamSeq::from(u32::MAX).next().is_err());
        Ok(())
    }
}
