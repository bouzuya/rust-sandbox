use std::str::FromStr;

use event_store_core::{Event as RawEvent, EventPayload, EventType as RawEventType};

use crate::{
    aggregate::user_request::value::user_response::UserResponse,
    event::EventType,
    value::{At, UserId, UserRequestId},
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
    user_id: String,
    user_request_id: String,
    status_code: u16,
    response_body: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRequestFinished {
    at: At,
    user_id: UserId,
    user_request_id: UserRequestId,
    user_response: UserResponse,
}

impl UserRequestFinished {
    pub(in crate::aggregate::user_request) fn new(
        at: At,
        user_id: UserId,
        user_request_id: UserRequestId,
        user_response: UserResponse,
    ) -> Self {
        Self {
            at,
            user_id,
            user_request_id,
            user_response,
        }
    }

    pub fn at(&self) -> At {
        self.at
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn user_request_id(&self) -> UserRequestId {
        self.user_request_id
    }

    pub fn user_response(&self) -> &UserResponse {
        &self.user_response
    }

    fn r#type() -> EventType {
        EventType::UserRequestFinished
    }
}

impl From<UserRequestFinished> for EventPayload {
    fn from(event: UserRequestFinished) -> Self {
        EventPayload::from_structured(&Payload {
            at: event.at.to_string(),
            user_id: event.user_id.to_string(),
            user_request_id: event.user_request_id.to_string(),
            status_code: event.user_response.status_code(),
            response_body: event.user_response.body().to_owned(),
        })
        .unwrap()
    }
}

impl TryFrom<RawEvent> for UserRequestFinished {
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
        let user_id = UserId::from_str(payload.user_id.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        let user_request_id = UserRequestId::from_str(payload.user_request_id.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        let user_response = UserResponse::new(payload.status_code, payload.response_body);
        Ok(Self::new(at, user_id, user_request_id, user_response))
    }
}

#[cfg(test)]
mod tests {
    use event_store_core::{EventId, EventStreamId, EventStreamSeq};

    use super::*;

    // TODO: test user_id
    // TODO: test user_request_id
    // TODO: test user_response

    #[test]
    fn raw_event_conversion_test() -> anyhow::Result<()> {
        let o = UserRequestFinished::new(
            At::from_str("2022-09-06T22:58:00.000000000Z")?,
            UserId::from_str("5464979d-8c47-47c7-9066-4cfee838c518")?,
            UserRequestId::from_str("9eb25b81-2df3-4502-81f4-668ea315c401")?,
            UserResponse::new(200, "{}".to_owned()),
        );
        let e = EventPayload::from_structured(&Payload {
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            user_id: "5464979d-8c47-47c7-9066-4cfee838c518".to_owned(),
            user_request_id: "9eb25b81-2df3-4502-81f4-668ea315c401".to_owned(),
            status_code: 200,
            response_body: "{}".to_owned(),
        })?;
        assert_eq!(EventPayload::from(o.clone()), e);
        assert_eq!(
            UserRequestFinished::try_from(RawEvent::new(
                EventId::generate(),
                RawEventType::from(UserRequestFinished::r#type()),
                EventStreamId::generate(),
                EventStreamSeq::from(1),
                e
            ))?,
            o
        );
        Ok(())
    }
}
