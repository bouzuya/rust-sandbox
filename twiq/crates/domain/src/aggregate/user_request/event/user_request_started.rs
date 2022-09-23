use std::str::FromStr;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
    Event as RawEvent, EventData, EventType,
};

use crate::value::At;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRequestStarted {
    pub(super) id: String,
    pub(super) at: String,
    pub(super) stream_id: String,
    pub(super) stream_seq: u32,
}

impl UserRequestStarted {
    pub(in crate::aggregate::user_request) fn new(
        id: EventId,
        at: At,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
    ) -> Self {
        Self {
            id: id.to_string(),
            at: at.to_string(),
            stream_id: stream_id.to_string(),
            stream_seq: u32::from(stream_seq),
        }
    }
}

impl From<UserRequestStarted> for RawEvent {
    fn from(event: UserRequestStarted) -> Self {
        RawEvent::new(
            EventId::from_str(event.id.as_str()).expect("id"),
            EventType::from_str("user_request_started").expect("event_type"),
            EventStreamId::from_str(event.stream_id.as_str()).expect("stream_id"),
            EventStreamSeq::from(event.stream_seq),
            EventData::try_from(serde_json::to_string(&event).expect("event")).expect("data"),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::aggregate::user_request::event::tests::serde_test;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let o = UserRequestStarted {
            id: "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8".to_owned(),
            at: "2022-09-06T22:58:00.000000000Z".to_owned(),
            stream_id: "a748c956-7e53-45ef-b1f0-1c52676a467c".to_owned(),
            stream_seq: 1,
        };
        let s = r#"{
  "id": "0ecb46f3-01a1-49b2-9405-0b4c40ecefe8",
  "at": "2022-09-06T22:58:00.000000000Z",
  "stream_id": "a748c956-7e53-45ef-b1f0-1c52676a467c",
  "stream_seq": 1
}"#;
        serde_test(o, s)?;
        Ok(())
    }
}
