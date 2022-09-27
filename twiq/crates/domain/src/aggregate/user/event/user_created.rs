use std::str::FromStr;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
    Event as RawEvent, EventData, EventType as RawEventType,
};

use crate::{
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

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserCreated {
    pub(super) id: String,
    pub(super) r#type: String,
    pub(super) at: String,
    pub(super) stream_id: String,
    pub(super) stream_seq: u32,
    pub(super) twitter_user_id: String,
}

impl UserCreated {
    pub(in crate::aggregate::user) fn new(
        id: EventId,
        at: At,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        twitter_user_id: TwitterUserId,
    ) -> Self {
        Self {
            id: id.to_string(),
            r#type: Self::r#type().to_string(),
            at: at.to_string(),
            stream_id: stream_id.to_string(),
            stream_seq: u32::from(stream_seq),
            twitter_user_id: twitter_user_id.to_string(),
        }
    }

    pub(in crate::aggregate::user) fn twitter_user_id(&self) -> TwitterUserId {
        TwitterUserId::from_str(&self.twitter_user_id).expect("twitter_user_id")
    }

    pub(in crate::aggregate::user) fn user_id(&self) -> UserId {
        UserId::from_str(&self.stream_id).expect("user_id")
    }

    fn r#type() -> EventType {
        EventType::UserCreated
    }
}

impl From<UserCreated> for RawEvent {
    fn from(event: UserCreated) -> Self {
        RawEvent::new(
            EventId::from_str(event.id.as_str()).expect("id"),
            RawEventType::from(UserCreated::r#type()),
            EventStreamId::from_str(event.stream_id.as_str()).expect("stream_id"),
            EventStreamSeq::from(event.stream_seq),
            EventData::try_from(serde_json::to_string(&event).expect("event")).expect("data"),
        )
    }
}

impl TryFrom<RawEvent> for UserCreated {
    type Error = Error;

    fn try_from(raw_event: RawEvent) -> Result<Self, Self::Error> {
        let event: Self = serde_json::from_str(raw_event.data().as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        if event.r#type != UserCreated::r#type().to_string() {
            return Err(Error::InvalidType);
        }
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use crate::event::tests::serde_test;

    use super::*;

    #[test]
    fn json_conversion_test() -> anyhow::Result<()> {
        let o = UserCreated {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            r#type: "user_created".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            twitter_user_id: "twitter_user_id1".to_owned(),
        };
        let s = r#"{
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "type": "user_created",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "twitter_user_id": "twitter_user_id1"
}"#;
        serde_test(o, s)?;
        Ok(())
    }

    #[test]
    fn raw_event_conversion_test() -> anyhow::Result<()> {
        let o = UserCreated {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            r#type: "user_created".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            twitter_user_id: "twitter_user_id1".to_owned(),
        };
        let e = RawEvent::new(
            EventId::from_str("0ecb46f3-01a1-49b2-9405-0b4c40ecefe8")?,
            RawEventType::from_str("user_created")?,
            EventStreamId::from_str("a748c956-7e53-45ef-b1f0-1c52676a467c")?,
            EventStreamSeq::from(1_u32),
            EventData::try_from(serde_json::to_string(&serde_json::from_str::<
                '_,
                UserCreated,
            >(
                r#"{
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "type": "user_created",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "twitter_user_id": "twitter_user_id1"
}"#,
            )?)?)?,
        );
        assert_eq!(RawEvent::from(o.clone()), e);
        assert_eq!(UserCreated::try_from(e)?, o);
        Ok(())
    }
}
