use std::str::FromStr;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
    Event as RawEvent, EventData, EventType as RawEventType,
};

use crate::{
    aggregate::user::value::twitter_user_name::TwitterUserName,
    event::EventType,
    value::{At, TwitterUserId},
};

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid type")]
    InvalidType,
    #[error("unknown {0}")]
    Unknown(String),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserUpdated {
    pub(super) id: String,
    pub(super) r#type: String,
    pub(super) at: String,
    pub(super) stream_id: String,
    pub(super) stream_seq: u32,
    pub(super) twitter_user_id: String,
    pub(super) twitter_user_name: String,
}

impl UserUpdated {
    pub(in crate::aggregate::user) fn new(
        id: EventId,
        at: At,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        twitter_user_id: TwitterUserId,
        twitter_user_name: TwitterUserName,
    ) -> Self {
        Self {
            id: id.to_string(),
            r#type: Self::r#type().to_string(),
            at: at.to_string(),
            stream_id: stream_id.to_string(),
            stream_seq: u32::from(stream_seq),
            twitter_user_id: twitter_user_id.to_string(),
            twitter_user_name: twitter_user_name.to_string(),
        }
    }

    pub(in crate::aggregate::user) fn at(&self) -> At {
        At::from_str(&self.at).expect("at")
    }

    pub(in crate::aggregate::user) fn stream_seq(&self) -> EventStreamSeq {
        EventStreamSeq::from(self.stream_seq)
    }

    fn r#type() -> EventType {
        EventType::UserUpdated
    }
}

impl From<UserUpdated> for RawEvent {
    fn from(event: UserUpdated) -> Self {
        RawEvent::new(
            EventId::from_str(event.id.as_str()).expect("id"),
            RawEventType::from(UserUpdated::r#type()),
            EventStreamId::from_str(event.stream_id.as_str()).expect("stream_id"),
            EventStreamSeq::from(event.stream_seq),
            EventData::try_from(serde_json::to_string(&event).expect("event")).expect("data"),
        )
    }
}

impl TryFrom<RawEvent> for UserUpdated {
    type Error = Error;

    fn try_from(raw_event: RawEvent) -> Result<Self, Self::Error> {
        let event: Self = serde_json::from_str(raw_event.data().as_str())
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

    #[test]
    fn json_conversion_test() -> anyhow::Result<()> {
        let o = UserUpdated {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            r#type: "user_updated".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            twitter_user_id: "twitter_user_id1".to_owned(),
            twitter_user_name: "twitter_user_name1".to_owned(),
        };
        let s = r#"{
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "type": "user_updated",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "twitter_user_id": "twitter_user_id1",
  "twitter_user_name": "twitter_user_name1"
}"#;
        serde_test(o, s)?;
        Ok(())
    }

    #[test]
    fn raw_event_conversion_test() -> anyhow::Result<()> {
        let o = UserUpdated {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            r#type: "user_updated".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            twitter_user_id: "twitter_user_id1".to_owned(),
            twitter_user_name: "twitter_user_name1".to_owned(),
        };
        let e = RawEvent::new(
            EventId::from_str("0ecb46f3-01a1-49b2-9405-0b4c40ecefe8")?,
            RawEventType::from_str("user_updated")?,
            EventStreamId::from_str("a748c956-7e53-45ef-b1f0-1c52676a467c")?,
            EventStreamSeq::from(1_u32),
            EventData::try_from(serde_json::to_string(&serde_json::from_str::<
                '_,
                UserUpdated,
            >(
                r#"{
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "type": "user_updated",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "twitter_user_id": "twitter_user_id1",
  "twitter_user_name": "twitter_user_name1"
}"#,
            )?)?)?,
        );
        assert_eq!(RawEvent::from(o.clone()), e);
        assert_eq!(UserUpdated::try_from(e)?, o);
        Ok(())
    }
}
