use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
};
use time::OffsetDateTime;

use super::value::twitter_user_id::TwitterUserId;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserCreated {
    id: String,
    at: String,
    stream_id: String,
    stream_seq: u32,
    twitter_user_id: String,
}

impl UserCreated {
    pub fn new(
        id: EventId,
        at: OffsetDateTime,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        twitter_user_id: TwitterUserId,
    ) -> Self {
        // TODO: check at timezone
        Self {
            id: id.to_string(),
            at: at.to_string(),
            stream_id: stream_id.to_string(),
            stream_seq: u32::from(stream_seq),
            twitter_user_id: twitter_user_id.to_string(),
        }
    }
}

pub struct UserFetchRequested;

pub struct UserUpdated;

pub enum Event {
    Created(UserCreated),
    Updated(UserUpdated),
    FetchRequested(UserFetchRequested),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_test() -> anyhow::Result<()> {
        // TODO
        Ok(())
    }

    #[test]
    fn user_created_test() -> anyhow::Result<()> {
        let deserialized: UserCreated = serde_json::from_str(
            r#"{"id":"0ecb46f3-01a1-49b2-9405-0b4c40ecefe8","at":"2022-09-06T22:58:00Z","stream_id":"a748c956-7e53-45ef-b1f0-1c52676a467c","stream_seq":1,"twitter_user_id":"twitter_user_id1"}"#,
        )?;
        assert_eq!(
            deserialized,
            UserCreated {
                id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
                at: "at1".to_owned(),
                stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
                stream_seq: 1,
                twitter_user_id: "twitter_user_id1".to_owned(),
            }
        );
        Ok(())
    }

    #[test]
    fn user_fetch_requested_test() -> anyhow::Result<()> {
        // TODO
        Ok(())
    }

    #[test]
    fn user_updated_test() -> anyhow::Result<()> {
        // TODO
        Ok(())
    }
}
