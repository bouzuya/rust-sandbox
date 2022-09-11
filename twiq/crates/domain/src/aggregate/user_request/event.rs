pub mod user_request_created;
pub mod user_request_started;

pub use self::user_request_created::UserRequestCreated;
pub use self::user_request_started::UserRequestStarted;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRequestFinished;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "user_request_created")]
    Created(UserRequestCreated),
    #[serde(rename = "user_request_started")]
    Started(UserRequestStarted),
    #[serde(rename = "user_request_finished")]
    Finished(UserRequestFinished),
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

impl_from_and_try_from!(Event::Created, UserRequestCreated);
impl_from_and_try_from!(Event::Started, UserRequestStarted);

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    use super::*;

    pub(in crate::aggregate::user_request::event) fn serde_test<T>(
        o: T,
        s: &str,
    ) -> anyhow::Result<()>
    where
        T: Debug + Eq + serde::de::DeserializeOwned + serde::Serialize,
    {
        let deserialized: T = serde_json::from_str(s)?;
        assert_eq!(deserialized, o);
        assert_eq!(serde_json::to_string_pretty(&o)?, s);
        Ok(())
    }

    #[test]
    fn user_request_created_test() -> anyhow::Result<()> {
        let o = Event::from(UserRequestCreated {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            twitter_user_id: "twitter_user_id1".to_owned(),
            user_id: "682106dd-b94c-4bd1-a808-e74b3d3fb56a".to_owned(),
        });
        let s = r#"{
  "type": "user_request_created",
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "twitter_user_id": "twitter_user_id1",
  "user_id": "682106dd-b94c-4bd1-a808-e74b3d3fb56a"
}"#;
        serde_test(o, s)?;
        Ok(())
    }

    #[test]
    fn user_request_started_test() -> anyhow::Result<()> {
        let o = Event::from(UserRequestStarted {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
        });
        let s = r#"{
  "type": "user_request_started",
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1
}"#;
        serde_test(o, s)?;
        Ok(())
    }
}
