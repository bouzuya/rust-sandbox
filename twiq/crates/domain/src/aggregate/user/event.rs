pub mod user_created;
pub mod user_requested;
pub mod user_updated;

pub use self::user_created::UserCreated;
pub use self::user_requested::UserRequested;
pub use self::user_updated::UserUpdated;

use event_store_core::Event as RawEvent;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
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
impl_from_and_try_from!(Event::Updated, UserUpdated);

impl From<Event> for RawEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::Created(e) => RawEvent::from(e),
            Event::Requested(e) => RawEvent::from(e),
            Event::Updated(e) => RawEvent::from(e),
        }
    }
}

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

    #[test]
    fn user_updated_test() -> anyhow::Result<()> {
        let o = Event::from(UserUpdated {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            twitter_user_id: "twitter_user_id1".to_owned(),
            twitter_user_name: "twitter_user_name1".to_owned(),
        });
        let s = r#"{
  "type": "user_updated",
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "twitter_user_id": "twitter_user_id1",
  "twitter_user_name": "twitter_user_name1"
}"#;
        serde_test(o, s)?;
        Ok(())
    }
}
