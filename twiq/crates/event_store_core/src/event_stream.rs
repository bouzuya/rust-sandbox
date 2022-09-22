use crate::{Event, EventStreamId, EventStreamSeq};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("empty")]
    Empty,
    #[error("invalid event stream id : {0}")]
    InvalidEventStreamId(EventStreamId),
    #[error("invalid event stream seq : {0}")]
    InvalidEventStreamSeq(EventStreamId),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventStream {
    events: Vec<Event>,
}

impl EventStream {
    pub fn new(events: Vec<Event>) -> Result<Self> {
        if events.is_empty() {
            return Err(Error::Empty);
        }

        let id = events[0].stream_id();
        let mut prev = events[0].stream_seq();
        for event in events.iter().skip(1) {
            if event.stream_id() != id {
                return Err(Error::InvalidEventStreamId(id));
            }

            let curr = event.stream_seq();
            if curr <= prev {
                return Err(Error::InvalidEventStreamSeq(id));
            }
            prev = curr;
        }

        Ok(Self { events })
    }

    pub fn id(&self) -> EventStreamId {
        self.events[0].stream_id()
    }

    pub fn seq(&self) -> EventStreamSeq {
        self.events
            .last()
            .expect("at least one exists")
            .stream_seq()
    }

    pub fn events(&self) -> Vec<Event> {
        self.events.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{EventData, EventId};

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        // empty
        assert!(EventStream::new(vec![]).is_err());

        // ok (an event)
        let event = Event::new(
            EventId::generate(),
            EventStreamId::generate(),
            EventStreamSeq::from(1_u32),
            EventData::from_str("{}")?,
        );
        let stream = EventStream::new(vec![event.clone()])?;
        assert_eq!(stream.id(), event.stream_id());
        assert_eq!(stream.seq(), event.stream_seq());
        assert_eq!(stream.events(), vec![event]);

        // invalid event_stream_id
        let event1 = Event::new(
            EventId::generate(),
            EventStreamId::generate(),
            EventStreamSeq::from(1_u32),
            EventData::from_str("{}")?,
        );
        let event2 = Event::new(
            EventId::generate(),
            EventStreamId::generate(),
            EventStreamSeq::from(2_u32),
            EventData::from_str("{}")?,
        );
        assert!(EventStream::new(vec![event1, event2]).is_err());

        // invalid event_stream_seq
        let stream_id = EventStreamId::generate();
        let event1 = Event::new(
            EventId::generate(),
            stream_id,
            EventStreamSeq::from(1_u32),
            EventData::from_str("{}")?,
        );
        let event2 = Event::new(
            EventId::generate(),
            stream_id,
            EventStreamSeq::from(1_u32),
            EventData::from_str("{}")?,
        );
        assert!(EventStream::new(vec![event1, event2]).is_err());

        // ok (two events)
        let stream_id = EventStreamId::generate();
        let event1 = Event::new(
            EventId::generate(),
            stream_id,
            EventStreamSeq::from(1_u32),
            EventData::from_str("{}")?,
        );
        let event2 = Event::new(
            EventId::generate(),
            stream_id,
            EventStreamSeq::from(2_u32),
            EventData::from_str("{}")?,
        );
        let stream = EventStream::new(vec![event1.clone(), event2.clone()])?;
        assert_eq!(stream.id(), event1.stream_id());
        assert_eq!(stream.seq(), event2.stream_seq());
        assert_eq!(stream.events(), vec![event1, event2]);
        Ok(())
    }
}
