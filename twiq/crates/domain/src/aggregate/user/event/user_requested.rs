use std::str::FromStr;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
    Event as RawEvent, EventData, EventType,
};

use crate::value::{At, TwitterUserId, UserId, UserRequestId};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRequested {
    pub(super) id: String,
    pub(super) r#type: String,
    pub(super) at: String,
    pub(super) stream_id: String,
    pub(super) stream_seq: u32,
    pub(super) twitter_user_id: String,
    pub(super) user_request_id: String,
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
            id: id.to_string(),
            r#type: crate::event::EventType::UserRequested.to_string(),
            at: at.to_string(),
            stream_id: stream_id.to_string(),
            stream_seq: u32::from(stream_seq),
            twitter_user_id: twitter_user_id.to_string(),
            user_request_id: user_request_id.to_string(),
        }
    }

    pub fn twitter_user_id(&self) -> TwitterUserId {
        TwitterUserId::from_str(&self.twitter_user_id).expect("twitter_user_id")
    }

    pub fn user_id(&self) -> UserId {
        UserId::from_str(&self.id).expect("user_id")
    }

    pub fn user_request_id(&self) -> UserRequestId {
        UserRequestId::from_str(&self.user_request_id).expect("user_request_id")
    }
}

impl From<UserRequested> for RawEvent {
    fn from(event: UserRequested) -> Self {
        RawEvent::new(
            EventId::from_str(event.id.as_str()).expect("id"),
            EventType::from(crate::event::EventType::UserRequested),
            EventStreamId::from_str(event.stream_id.as_str()).expect("stream_id"),
            EventStreamSeq::from(event.stream_seq),
            EventData::try_from(serde_json::to_string(&event).expect("event")).expect("data"),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::aggregate::user::event::tests::serde_test;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let o = UserRequested {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            r#type: "user_requested".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            twitter_user_id: "twitter_user_id1".to_owned(),
            user_request_id: "868aecdc-d860-4232-8000-69e4623f1317".to_owned(),
        };
        let s = r#"{
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "type": "user_requested",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "twitter_user_id": "twitter_user_id1",
  "user_request_id": "868aecdc-d860-4232-8000-69e4623f1317"
}"#;
        serde_test(o, s)?;
        Ok(())
    }

    // TODO: twitter_user_id test
    // TODO: user_id test
    // TODO: user_request_id test
}
