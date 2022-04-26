use super::{event_stream_id::EventStreamId, event_stream_version::EventStreamVersion};

#[derive(Clone, Debug)]
pub struct Event {
    pub stream_id: EventStreamId,
    pub data: String,
    pub version: EventStreamVersion,
}
