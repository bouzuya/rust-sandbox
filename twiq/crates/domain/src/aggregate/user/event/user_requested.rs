use std::str::FromStr;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
    Event as RawEvent, EventPayload, EventType as RawEventType,
};

use crate::{
    event::EventType,
    value::{At, TwitterUserId, UserId, UserRequestId},
};

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid type")]
    InvalidType,
    #[error("unknown {0}")]
    Unknown(String),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
struct Payload {
    at: String,
    twitter_user_id: String,
    user_request_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRequested {
    event: RawEvent,
    at: At,
    twitter_user_id: TwitterUserId,
    user_request_id: UserRequestId,
}

impl UserRequested {
    pub(in crate::aggregate::user) fn new(
        id: EventId,
        at: At,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        twitter_user_id: TwitterUserId,
        user_request_id: UserRequestId,
    ) -> UserRequested {
        Self {
            event: RawEvent::new(
                id,
                RawEventType::from(Self::r#type()),
                stream_id,
                stream_seq,
                EventPayload::from_structured(&Payload {
                    at: at.to_string(),
                    twitter_user_id: twitter_user_id.to_string(),
                    user_request_id: user_request_id.to_string(),
                })
                .expect("event_payload"),
            ),
            at,
            twitter_user_id,
            user_request_id,
        }
    }

    pub(in crate::aggregate::user) fn at(&self) -> At {
        self.at
    }

    pub(in crate::aggregate::user) fn stream_seq(&self) -> EventStreamSeq {
        self.event.stream_seq()
    }

    pub fn twitter_user_id(&self) -> &TwitterUserId {
        &self.twitter_user_id
    }

    pub fn user_id(&self) -> UserId {
        UserId::from(self.event.stream_id())
    }

    pub fn user_request_id(&self) -> UserRequestId {
        self.user_request_id
    }

    fn r#type() -> EventType {
        EventType::UserRequested
    }
}

impl From<UserRequested> for RawEvent {
    fn from(event: UserRequested) -> Self {
        event.event
    }
}

impl TryFrom<RawEvent> for UserRequested {
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
        let twitter_user_id = TwitterUserId::from_str(payload.twitter_user_id.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        let user_request_id = UserRequestId::from_str(payload.user_request_id.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        Ok(Self::new(
            raw_event.id(),
            at,
            raw_event.stream_id(),
            raw_event.stream_seq(),
            twitter_user_id,
            user_request_id,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_event_conversion_test() -> anyhow::Result<()> {
        let o = UserRequested::new(
            EventId::from_str("0ecb46f3-01a1-49b2-9405-0b4c40ecefe8")?,
            At::from_str("2022-09-06T22:58:00.000000000Z")?,
            EventStreamId::from_str("a748c956-7e53-45ef-b1f0-1c52676a467c")?,
            EventStreamSeq::from(1),
            TwitterUserId::from_str("twitter_user_id1")?,
            UserRequestId::from_str("868aecdc-d860-4232-8000-69e4623f1317")?,
        );
        let e = RawEvent::new(
            EventId::from_str("0ecb46f3-01a1-49b2-9405-0b4c40ecefe8")?,
            RawEventType::from_str("user_requested")?,
            EventStreamId::from_str("a748c956-7e53-45ef-b1f0-1c52676a467c")?,
            EventStreamSeq::from(1_u32),
            EventPayload::from_structured(&Payload {
                at: "2022-09-06T22:58:00.000000000Z".to_owned(),
                twitter_user_id: "twitter_user_id1".to_owned(),
                user_request_id: "868aecdc-d860-4232-8000-69e4623f1317".to_owned(),
            })?,
        );
        assert_eq!(RawEvent::from(o.clone()), e);
        assert_eq!(UserRequested::try_from(e)?, o);
        Ok(())
    }

    // TODO: twitter_user_id test
    // TODO: user_id test
    // TODO: user_request_id test
}
