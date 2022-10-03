use std::str::FromStr;

use event_store_core::{Event as RawEvent, EventPayload, EventType as RawEventType};

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
    user_id: String,
    user_request_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRequested {
    at: At,
    twitter_user_id: TwitterUserId,
    user_id: UserId,
    user_request_id: UserRequestId,
}

impl UserRequested {
    pub(in crate::aggregate::user) fn new(
        at: At,
        twitter_user_id: TwitterUserId,
        user_id: UserId,
        user_request_id: UserRequestId,
    ) -> UserRequested {
        Self {
            at,
            twitter_user_id,
            user_id,
            user_request_id,
        }
    }

    pub(in crate::aggregate::user) fn at(&self) -> At {
        self.at
    }

    pub fn twitter_user_id(&self) -> &TwitterUserId {
        &self.twitter_user_id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn user_request_id(&self) -> UserRequestId {
        self.user_request_id
    }

    pub(in crate::aggregate::user) fn r#type() -> EventType {
        EventType::UserRequested
    }
}

impl From<UserRequested> for EventPayload {
    fn from(event: UserRequested) -> Self {
        EventPayload::from_structured(&Payload {
            at: event.at.to_string(),
            twitter_user_id: event.twitter_user_id.to_string(),
            user_id: event.user_id.to_string(),
            user_request_id: event.user_id.to_string(),
        })
        .unwrap()
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
        let user_id = UserId::from_str(payload.user_id.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        let user_request_id = UserRequestId::from_str(payload.user_request_id.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        Ok(Self::new(at, twitter_user_id, user_id, user_request_id))
    }
}

#[cfg(test)]
mod tests {
    use event_store_core::{EventId, EventStreamId, EventStreamSeq};

    use super::*;

    #[test]
    fn raw_event_conversion_test() -> anyhow::Result<()> {
        let o = UserRequested::new(
            At::from_str("2022-09-06T22:58:00.000000000Z")?,
            TwitterUserId::from_str("twitter_user_id1")?,
            UserId::from_str("c274a425-baed-4252-9f92-ed8d7e84a096")?,
            UserRequestId::from_str("868aecdc-d860-4232-8000-69e4623f1317")?,
        );
        let e = EventPayload::from_structured(&Payload {
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            twitter_user_id: "twitter_user_id1".to_owned(),
            user_id: "c274a425-baed-4252-9f92-ed8d7e84a096".to_owned(),
            user_request_id: "868aecdc-d860-4232-8000-69e4623f1317".to_owned(),
        })?;
        assert_eq!(EventPayload::from(o.clone()), e);
        assert_eq!(
            UserRequested::try_from(RawEvent::new(
                EventId::generate(),
                RawEventType::from(UserRequested::r#type()),
                EventStreamId::generate(),
                EventStreamSeq::from(1),
                e
            ))?,
            o
        );
        Ok(())
    }

    // TODO: twitter_user_id test
    // TODO: user_id test
    // TODO: user_request_id test
}
