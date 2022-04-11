use super::{aggregate_id::AggregateId, aggregate_version::AggregateVersion};

#[derive(Clone, Debug)]
pub struct Event {
    pub event_stream_id: AggregateId,
    pub data: String,
    pub version: AggregateVersion,
}
