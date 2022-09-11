use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
};

use crate::value::{At, TwitterUserId, UserId};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRequestCreated {
    pub(super) id: String,
    pub(super) at: String,
    pub(super) stream_id: String,
    pub(super) stream_seq: u32,
    pub(super) twitter_user_id: String,
    pub(super) user_id: String,
}

impl UserRequestCreated {
    pub(in crate::aggregate::user_request) fn new(
        id: EventId,
        at: At,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        twitter_user_id: TwitterUserId,
        user_id: UserId,
    ) -> Self {
        Self {
            id: id.to_string(),
            at: at.to_string(),
            stream_id: stream_id.to_string(),
            stream_seq: u32::from(stream_seq),
            twitter_user_id: twitter_user_id.to_string(),
            user_id: user_id.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::aggregate::user_request::event::tests::serde_test;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let o = UserRequestCreated {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            twitter_user_id: "twitter_user_id1".to_owned(),
            user_id: "682106dd-b94c-4bd1-a808-e74b3d3fb56a".to_owned(),
        };
        let s = r#"{
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "twitter_user_id": "twitter_user_id1"
  "user_id": "682106dd-b94c-4bd1-a808-e74b3d3fb56a"
}"#;
        serde_test(o, s)?;
        Ok(())
    }
}
