use crate::{Event, EventStreamId};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid event stream id : {0}")]
    InvalidEventStreamId(EventStreamId),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Eq, PartialEq)]
pub struct EventStream {
    id: EventStreamId,
    events: Vec<Event>,
}

impl EventStream {
    pub fn new(id: EventStreamId, events: Vec<Event>) -> Result<Self> {
        // TODO: check
        Ok(Self { id, events })
    }

    pub fn id(&self) -> EventStreamId {
        self.id
    }

    pub fn events(&self) -> Vec<Event> {
        self.events.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: test id
    // TODO: test events
    // TODO: test new
}
