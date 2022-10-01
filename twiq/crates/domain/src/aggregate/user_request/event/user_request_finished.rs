use std::str::FromStr;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
    Event as RawEvent, EventPayload, EventType as RawEventType,
};

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
pub struct UserRequestFinished {
    pub(super) id: String,
    pub(super) r#type: String,
    pub(super) at: String,
    pub(super) stream_id: String,
    pub(super) stream_seq: u32,
    pub(super) user_id: String,
    pub(super) status_code: u16,
    pub(super) response_body: String,
}

impl UserRequestFinished {
    pub(in crate::aggregate::user_request) fn new(
        id: EventId,
        at: At,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        user_id: UserId,
        user_response: UserResponse,
    ) -> Self {
        Self {
            id: id.to_string(),
            r#type: Self::r#type().to_string(),
            at: at.to_string(),
            stream_id: stream_id.to_string(),
            stream_seq: u32::from(stream_seq),
            user_id: user_id.to_string(),
            status_code: user_response.status_code(),
            response_body: user_response.body().to_owned(),
        }
    }

    pub fn at(&self) -> At {
        At::from_str(&self.at).expect("at")
    }

    pub fn user_id(&self) -> UserId {
        UserId::from_str(&self.user_id).expect("user_id")
    }

    pub fn user_request_id(&self) -> UserRequestId {
        UserRequestId::from_str(&self.stream_id).expect("user_request_id")
    }

    pub fn user_response(&self) -> UserResponse {
        UserResponse::new(self.status_code, self.response_body.clone())
    }

    fn r#type() -> EventType {
        EventType::UserRequestFinished
    }
}

impl From<UserRequestFinished> for RawEvent {
    fn from(event: UserRequestFinished) -> Self {
        RawEvent::new(
            EventId::from_str(event.id.as_str()).expect("id"),
            RawEventType::from(UserRequestFinished::r#type()),
            EventStreamId::from_str(event.stream_id.as_str()).expect("stream_id"),
            EventStreamSeq::from(event.stream_seq),
            EventPayload::try_from(serde_json::to_string(&event).expect("event")).expect("data"),
        )
    }
}

impl TryFrom<RawEvent> for UserRequestFinished {
    type Error = Error;

    fn try_from(raw_event: RawEvent) -> Result<Self, Self::Error> {
        let event: Self = serde_json::from_str(raw_event.payload().as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        if event.r#type != Self::r#type().to_string() {
            return Err(Error::InvalidType);
        }
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use crate::event::tests::serde_test;

    use super::*;

    // TODO: test user_id
    // TODO: test user_request_id
    // TODO: test user_response

    #[test]
    fn json_conversion_test() -> anyhow::Result<()> {
        let o = UserRequestFinished {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            r#type: "user_request_finished".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            user_id: "5464979d-8c47-47c7-9066-4cfee838c518".to_owned(),
            status_code: 200,
            response_body: "{}".to_owned(),
        };
        let s = r#"{
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "type": "user_request_finished",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "user_id": "5464979d-8c47-47c7-9066-4cfee838c518",
  "status_code": 200,
  "response_body": "{}"
}"#;
        serde_test(o, s)?;
        Ok(())
    }

    #[test]
    fn raw_event_conversion_test() -> anyhow::Result<()> {
        let o = UserRequestFinished {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            r#type: "user_request_finished".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            user_id: "5464979d-8c47-47c7-9066-4cfee838c518".to_owned(),
            status_code: 200,
            response_body: "{}".to_owned(),
        };
        let e = RawEvent::new(
            EventId::from_str("0ecb46f3-01a1-49b2-9405-0b4c40ecefe8")?,
            RawEventType::from_str("user_request_finished")?,
            EventStreamId::from_str("a748c956-7e53-45ef-b1f0-1c52676a467c")?,
            EventStreamSeq::from(1_u32),
            EventPayload::try_from(serde_json::to_string(&serde_json::from_str::<
                '_,
                UserRequestFinished,
            >(
                r#"{
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "type": "user_request_finished",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "user_id": "5464979d-8c47-47c7-9066-4cfee838c518",
  "status_code": 200,
  "response_body": "{}"
}"#,
            )?)?)?,
        );
        assert_eq!(RawEvent::from(o.clone()), e);
        assert_eq!(UserRequestFinished::try_from(e)?, o);
        Ok(())
    }
}
