use super::{aggregate_version::AggregateVersion, event_stream_id::EventStreamId};

#[derive(Clone, Debug)]
pub struct Event {
    pub event_stream_id: EventStreamId,
    pub data: String,
    pub version: AggregateVersion,
}
