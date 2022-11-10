use std::str::FromStr;

use event_store_core::{Event as RawEvent, EventPayload, EventType as RawEventType};
use user_request_id::UserRequestId;

use crate::{
    event::EventType,
    value::{user_request_id, At},
};

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid type")]
    InvalidType,
    #[error("unknown {0}")]
    Unknown(String),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct Payload {
    at: String,
    user_request_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRequestStarted {
    at: At,
    user_request_id: UserRequestId,
}

impl UserRequestStarted {
    pub(in crate::aggregate::user_request) fn new(at: At, user_request_id: UserRequestId) -> Self {
        Self {
            at,
            user_request_id,
        }
    }

    pub fn r#type() -> EventType {
        EventType::UserRequestStarted
    }
}

impl From<UserRequestStarted> for EventPayload {
    fn from(event: UserRequestStarted) -> Self {
        EventPayload::from_structured(&Payload {
            at: event.at.to_string(),
            user_request_id: event.user_request_id.to_string(),
        })
        .unwrap()
    }
}

impl TryFrom<RawEvent> for UserRequestStarted {
    type Error = Error;

    fn try_from(raw_event: RawEvent) -> Result<Self, Self::Error> {
        if raw_event.r#type() != &RawEventType::from(Self::r#type()) {
            return Err(Error::InvalidType);
        }
        let payload: Payload = raw_event
            .payload()
            .to_structured()
            .map_err(|e| Error::Unknown(e.to_string()))?;
        let at = At::from_str(payload.at.as_str()).map_err(|e| Error::Unknown(e.to_string()))?;
        let user_request_id = UserRequestId::from_str(payload.user_request_id.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        Ok(Self::new(at, user_request_id))
    }
}

#[cfg(test)]
mod tests {
    use event_store_core::{EventAt, EventId, EventStreamId, EventStreamSeq};

    use super::*;

    #[test]
    fn raw_event_conversion_test() -> anyhow::Result<()> {
        let o = UserRequestStarted::new(
            At::from_str("2022-09-06T22:58:00.000000000Z")?,
            UserRequestId::from_str("9eb25b81-2df3-4502-81f4-668ea315c401")?,
        );
        let e = EventPayload::from_structured(&Payload {
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            user_request_id: "9eb25b81-2df3-4502-81f4-668ea315c401".to_owned(),
        })?;
        assert_eq!(EventPayload::from(o.clone()), e);
        assert_eq!(
            UserRequestStarted::try_from(RawEvent::new(
                EventId::generate(),
                RawEventType::from(UserRequestStarted::r#type()),
                EventStreamId::generate(),
                EventStreamSeq::from(1),
                EventAt::now(),
                e
            ))?,
            o
        );
        Ok(())
    }
}
