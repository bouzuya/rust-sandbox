use super::{event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq};

#[derive(Clone, Debug)]
pub struct Event {
    pub id: EventId,
    pub stream_id: EventStreamId,
    pub stream_seq: EventStreamSeq,
    pub data: String,
}
