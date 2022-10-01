use std::str::FromStr;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
    Event as RawEvent, EventPayload, EventType as RawEventType,
};

use crate::{
    aggregate::user::value::twitter_user_name::TwitterUserName,
    event::EventType,
    value::{At, TwitterUserId, UserId},
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
    twitter_user_name: String,
    user_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserUpdated {
    event: RawEvent,
    at: At,
    twitter_user_id: TwitterUserId,
    twitter_user_name: TwitterUserName,
    user_id: UserId,
}

impl UserUpdated {
    pub(in crate::aggregate::user) fn new(
        id: EventId,
        at: At,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        twitter_user_id: TwitterUserId,
        twitter_user_name: TwitterUserName,
        user_id: UserId,
    ) -> Self {
        Self {
            event: RawEvent::new(
                id,
                RawEventType::from(Self::r#type()),
                stream_id,
                stream_seq,
                EventPayload::from_structured(&Payload {
                    at: at.to_string(),
                    twitter_user_id: twitter_user_id.to_string(),
                    twitter_user_name: twitter_user_name.to_string(),
                    user_id: user_id.to_string(),
                })
                .expect("event_payload"),
            ),
            at,
            twitter_user_id,
            twitter_user_name,
            user_id,
        }
    }

    pub(in crate::aggregate::user) fn at(&self) -> At {
        self.at
    }

    pub(in crate::aggregate::user) fn stream_seq(&self) -> EventStreamSeq {
        self.event.stream_seq()
    }

    fn r#type() -> EventType {
        EventType::UserUpdated
    }
}

impl From<UserUpdated> for RawEvent {
    fn from(event: UserUpdated) -> Self {
        event.event
    }
}

impl TryFrom<RawEvent> for UserUpdated {
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
        let twitter_user_name = TwitterUserName::from_str(payload.twitter_user_name.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        let user_id = UserId::from_str(payload.user_id.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        Ok(Self::new(
            raw_event.id(),
            at,
            raw_event.stream_id(),
            raw_event.stream_seq(),
            twitter_user_id,
            twitter_user_name,
            user_id,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_event_conversion_test() -> anyhow::Result<()> {
        let o = UserUpdated::new(
            EventId::from_str("0ecb46f3-01a1-49b2-9405-0b4c40ecefe8")?,
            At::from_str("2022-09-06T22:58:00.000000000Z")?,
            EventStreamId::from_str("a748c956-7e53-45ef-b1f0-1c52676a467c")?,
            EventStreamSeq::from(1),
            TwitterUserId::from_str("twitter_user_id1")?,
            TwitterUserName::from_str("twitter_user_name1")?,
            UserId::from_str("c274a425-baed-4252-9f92-ed8d7e84a096")?,
        );
        let e = RawEvent::new(
            EventId::from_str("0ecb46f3-01a1-49b2-9405-0b4c40ecefe8")?,
            RawEventType::from_str("user_updated")?,
            EventStreamId::from_str("a748c956-7e53-45ef-b1f0-1c52676a467c")?,
            EventStreamSeq::from(1_u32),
            EventPayload::from_structured(&Payload {
                at: "2022-09-06T22:58:00.000000000Z".to_owned(),
                twitter_user_id: "twitter_user_id1".to_owned(),
                twitter_user_name: "twitter_user_name1".to_owned(),
                user_id: "c274a425-baed-4252-9f92-ed8d7e84a096".to_owned(),
            })?,
        );
        assert_eq!(RawEvent::from(o.clone()), e);
        assert_eq!(UserUpdated::try_from(e)?, o);
        Ok(())
    }
}
