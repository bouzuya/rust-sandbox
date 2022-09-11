pub mod user_created;
pub mod user_requested;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
};

pub use self::user_created::UserCreated;
pub use self::user_requested::UserRequested;

use super::value::{at::At, twitter_user_id::TwitterUserId, twitter_user_name::TwitterUserName};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserUpdated {
    id: String,
    at: String,
    stream_id: String,
    stream_seq: u32,
    twitter_user_id: String,
    twitter_user_name: String,
}

impl UserUpdated {
    pub fn new(
        id: EventId,
        at: At,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        twitter_user_id: TwitterUserId,
        twitter_user_name: TwitterUserName,
    ) -> Self {
        Self {
            id: id.to_string(),
            at: at.to_string(),
            stream_id: stream_id.to_string(),
            stream_seq: u32::from(stream_seq),
            twitter_user_id: twitter_user_id.to_string(),
            twitter_user_name: twitter_user_name.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "user_created")]
    Created(UserCreated),
    #[serde(rename = "user_requested")]
    Requested(UserRequested),
    #[serde(rename = "user_updated")]
    Updated(UserUpdated),
}

macro_rules! impl_from_and_try_from {
    ($constructor: path, $ty: ty) => {
        impl From<$ty> for Event {
            fn from(value: $ty) -> Self {
                $constructor(value)
            }
        }

        impl TryFrom<Event> for $ty {
            type Error = Error;

            fn try_from(value: Event) -> Result<Self, Self::Error> {
                if let $constructor(value) = value {
                    Ok(value)
                } else {
                    Err(Error::Unknown("try from failed".to_owned()))
                }
            }
        }
    };
}

impl_from_and_try_from!(Event::Created, UserCreated);
impl_from_and_try_from!(Event::Requested, UserRequested);

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    use super::*;

    pub(in crate::aggregate::user::event) fn serde_test<T>(o: T, s: &str) -> anyhow::Result<()>
    where
        T: Debug + Eq + serde::de::DeserializeOwned + serde::Serialize,
    {
        let deserialized: T = serde_json::from_str(s)?;
        assert_eq!(deserialized, o);
        assert_eq!(serde_json::to_string_pretty(&o)?, s);
        Ok(())
    }

    #[test]
    fn user_created_test() -> anyhow::Result<()> {
        let o = Event::from(UserCreated {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            twitter_user_id: "twitter_user_id1".to_owned(),
        });
        let s = r#"{
  "type": "user_created",
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "twitter_user_id": "twitter_user_id1"
}"#;
        serde_test(o, s)?;
        Ok(())
    }

    #[test]
    fn user_requested_test() -> anyhow::Result<()> {
        let o = Event::from(UserRequested {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            twitter_user_id: "twitter_user_id1".to_owned(),
            user_request_id: "868aecdc-d860-4232-8000-69e4623f1317".to_owned(),
        });
        let s = r#"{
  "type": "user_requested",
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "twitter_user_id": "twitter_user_id1",
  "user_request_id": "868aecdc-d860-4232-8000-69e4623f1317"
}"#;
        serde_test(o, s)?;
        Ok(())
    }
}
