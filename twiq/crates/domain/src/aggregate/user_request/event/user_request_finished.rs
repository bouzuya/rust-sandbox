use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
};

use crate::value::At;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRequestFinished {
    pub(super) id: String,
    pub(super) at: String,
    pub(super) stream_id: String,
    pub(super) stream_seq: u32,
    pub(super) status_code: u16,
    pub(super) response_body: String,
}

impl UserRequestFinished {
    pub(in crate::aggregate::user_request) fn new(
        id: EventId,
        at: At,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        status_code: u16,
        response_body: String,
    ) -> Self {
        Self {
            id: id.to_string(),
            at: at.to_string(),
            stream_id: stream_id.to_string(),
            stream_seq: u32::from(stream_seq),
            status_code,
            response_body,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::aggregate::user_request::event::tests::serde_test;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let o = UserRequestFinished {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
            status_code: 200,
            response_body: "{}".to_owned(),
        };
        let s = r#"{
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1,
  "status_code": 200,
  "response_body": "{}"
}"#;
        serde_test(o, s)?;
        Ok(())
    }
}
