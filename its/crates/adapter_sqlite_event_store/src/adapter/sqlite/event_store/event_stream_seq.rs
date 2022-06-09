use super::error::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventStreamSeq(u32);

impl From<EventStreamSeq> for i64 {
    fn from(version: EventStreamSeq) -> Self {
        i64::from(version.0)
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
        let value = u32::try_from(value).map_err(|_| Error::InvalidEventStreamVersion)?;
        Ok(Self(value))
    }
}
