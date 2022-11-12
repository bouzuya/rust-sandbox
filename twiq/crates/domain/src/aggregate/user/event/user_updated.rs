use std::str::FromStr;

use event_store_core::{Event as RawEvent, EventPayload, EventType as RawEventType};

use crate::{
    aggregate::user::value::twitter_user_name::TwitterUserName,
    event::EventType,
    value::{TwitterUserId, UserId},
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
    twitter_user_id: String,
    twitter_user_name: String,
    user_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserUpdated {
    twitter_user_id: TwitterUserId,
    twitter_user_name: TwitterUserName,
    user_id: UserId,
}

impl UserUpdated {
    pub(in crate::aggregate::user) fn new(
        twitter_user_id: TwitterUserId,
        twitter_user_name: TwitterUserName,
        user_id: UserId,
    ) -> Self {
        Self {
            twitter_user_id,
            twitter_user_name,
            user_id,
        }
    }

    pub(in crate::aggregate::user) fn r#type() -> EventType {
        EventType::UserUpdated
    }

    pub fn twitter_user_id(&self) -> &TwitterUserId {
        &self.twitter_user_id
    }

    pub fn twitter_user_name(&self) -> &TwitterUserName {
        &self.twitter_user_name
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }
}

impl From<UserUpdated> for EventPayload {
    fn from(event: UserUpdated) -> Self {
        EventPayload::from_structured(&Payload {
            twitter_user_id: event.twitter_user_id.to_string(),
            twitter_user_name: event.twitter_user_name.to_string(),
            user_id: event.user_id.to_string(),
        })
        .unwrap()
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
        let twitter_user_id = TwitterUserId::from_str(payload.twitter_user_id.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        let twitter_user_name = TwitterUserName::from_str(payload.twitter_user_name.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        let user_id = UserId::from_str(payload.user_id.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        Ok(Self::new(twitter_user_id, twitter_user_name, user_id))
    }
}

#[cfg(test)]
mod tests {
    use event_store_core::{EventAt, EventId, EventStreamId, EventStreamSeq};

    use super::*;

    #[test]
    fn raw_event_conversion_test() -> anyhow::Result<()> {
        let o = UserUpdated::new(
            TwitterUserId::from_str("twitter_user_id1")?,
            TwitterUserName::from_str("twitter_user_name1")?,
            UserId::from_str("c274a425-baed-4252-9f92-ed8d7e84a096")?,
        );
        let e = EventPayload::from_structured(&Payload {
            twitter_user_id: "twitter_user_id1".to_owned(),
            twitter_user_name: "twitter_user_name1".to_owned(),
            user_id: "c274a425-baed-4252-9f92-ed8d7e84a096".to_owned(),
        })?;
        assert_eq!(EventPayload::from(o.clone()), e);
        assert_eq!(
            UserUpdated::try_from(RawEvent::new(
                EventId::generate(),
                RawEventType::from(UserUpdated::r#type()),
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
