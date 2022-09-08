use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
};

use super::value::{
    at::At, twitter_user_id::TwitterUserId, twitter_user_name::TwitterUserName,
    user_request_id::UserRequestId,
};

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
        at: At,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        twitter_user_id: TwitterUserId,
    ) -> Self {
        Self {
            id: id.to_string(),
            at: at.to_string(),
            stream_id: stream_id.to_string(),
            stream_seq: u32::from(stream_seq),
            twitter_user_id: twitter_user_id.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserFetchRequested {
    id: String,
    at: String,
    stream_id: String,
    stream_seq: u32,
    twitter_user_id: String,
    user_request_id: String,
}

impl UserFetchRequested {
    pub(crate) fn new(
        id: EventId,
        at: At,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        twitter_user_id: TwitterUserId,
        user_request_id: UserRequestId,
    ) -> UserFetchRequested {
        Self {
            id: id.to_string(),
            at: at.to_string(),
            stream_id: stream_id.to_string(),
            stream_seq: u32::from(stream_seq),
            twitter_user_id: twitter_user_id.to_string(),
            user_request_id: user_request_id.to_string(),
        }
    }
}

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
            r#"{"id":"0ecb46f3-01a1-49b2-9405-0b4c40ecefe8","at":"2022-09-06T22:58:00.000000000Z","stream_id":"a748c956-7e53-45ef-b1f0-1c52676a467c","stream_seq":1,"twitter_user_id":"twitter_user_id1"}"#,
        )?;
        assert_eq!(
            deserialized,
            UserCreated {
                id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
                at: "2022-09-06T22:58:00.000000000Z".to_owned(),
                stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
                stream_seq: 1,
                twitter_user_id: "twitter_user_id1".to_owned(),
            }
        );
        Ok(())
    }

    #[test]
    fn user_fetch_requested_test() -> anyhow::Result<()> {
        let deserialized: UserFetchRequested = serde_json::from_str(
            r#"{"id":"0ecb46f3-01a1-49b2-9405-0b4c40ecefe8","at":"2022-09-06T22:58:00.000000000Z","stream_id":"a748c956-7e53-45ef-b1f0-1c52676a467c","stream_seq":1,"twitter_user_id":"twitter_user_id1","user_request_id":"868aecdc-d860-4232-8000-69e4623f1317"}"#,
        )?;
        assert_eq!(
            deserialized,
            UserFetchRequested {
                id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
                at: "2022-09-06T22:58:00.000000000Z".to_owned(),
                stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
                stream_seq: 1,
                twitter_user_id: "twitter_user_id1".to_owned(),
                user_request_id: "868aecdc-d860-4232-8000-69e4623f1317".to_owned()
            }
        );
        Ok(())
    }

    #[test]
    fn user_updated_test() -> anyhow::Result<()> {
        let deserialized: UserUpdated = serde_json::from_str(
            r#"{"id":"0ecb46f3-01a1-49b2-9405-0b4c40ecefe8","at":"2022-09-06T22:58:00.000000000Z","stream_id":"a748c956-7e53-45ef-b1f0-1c52676a467c","stream_seq":1,"twitter_user_id":"twitter_user_id1","twitter_user_name":"bouzuya"}"#,
        )?;
        assert_eq!(
            deserialized,
            UserUpdated {
                id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
                at: "2022-09-06T22:58:00.000000000Z".to_owned(),
                stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
                stream_seq: 1,
                twitter_user_id: "twitter_user_id1".to_owned(),
                twitter_user_name: "bouzuya".to_owned()
            }
        );
        Ok(())
    }
}
