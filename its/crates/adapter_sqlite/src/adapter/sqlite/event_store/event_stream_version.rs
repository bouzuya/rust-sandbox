use super::event_store_error::EventStoreError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventStreamVersion(u32);

impl From<EventStreamVersion> for i64 {
    fn from(version: EventStreamVersion) -> Self {
        i64::from(version.0)
    }
}

impl From<u32> for EventStreamVersion {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<i64> for EventStreamVersion {
    type Error = EventStoreError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value = u32::try_from(value).map_err(|_| EventStoreError::InvalidEventStreamVersion)?;
        Ok(Self(value))
    }
}
