use crate::{
    event_type::EventType, Event, EventAt, EventId, EventPayload, EventStreamId, EventStreamSeq,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("empty")]
    Empty,
    #[error("invalid event stream id : {0}")]
    InvalidEventStreamId(EventStreamId),
    #[error("invalid event stream seq : {0}")]
    InvalidEventStreamSeq(EventStreamId),
    #[error("overflow event stream seq : {0}")]
    OverflowEventStreamSeq(EventStreamId),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventStream {
    events: Vec<Event>,
}

impl EventStream {
    pub fn generate<T, U>(event_type: T, event_payload: U) -> Self
    where
        T: Into<EventType>,
        U: Into<EventPayload>,
    {
        Self {
            events: vec![Event::new(
                EventId::generate(),
                event_type.into(),
                EventStreamId::generate(),
                EventStreamSeq::from(1_u32),
                EventAt::now(),
                event_payload.into(),
            )],
        }
    }

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

    pub fn push<T, U>(&mut self, event_type: T, event_payload: U) -> Result<()>
    where
        T: Into<EventType>,
        U: Into<EventPayload>,
    {
        self.events.push(Event::new(
            EventId::generate(),
            event_type.into(),
            self.id(),
            self.seq()
                .next()
                .map_err(|_| Error::OverflowEventStreamSeq(self.id()))?,
            EventAt::now(),
            event_payload.into(),
        ));
        Ok(())
    }

    pub fn push_event(&mut self, event: Event) -> Result<()> {
        if event.stream_id() != self.id() {
            return Err(Error::InvalidEventStreamId(self.id()));
        }
        if event.stream_seq() <= self.seq() {
            return Err(Error::InvalidEventStreamId(self.id()));
        }
        self.events.push(event);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::EventPayload;

    use super::*;

    #[test]
    fn generate_test() -> anyhow::Result<()> {
        let stream = EventStream::generate(
            EventType::from_str("created")?,
            EventPayload::from_str("{}")?,
        );
        assert_eq!(stream.seq(), EventStreamSeq::from(1_u32));
        Ok(())
    }

    #[test]
    fn new_test_empty() -> anyhow::Result<()> {
        assert!(EventStream::new(vec![]).is_err());
        Ok(())
    }

    #[test]
    fn new_test_an_event() -> anyhow::Result<()> {
        let event = Event::new(
            EventId::generate(),
            EventType::from_str("created")?,
            EventStreamId::generate(),
            EventStreamSeq::from(1_u32),
            EventAt::now(),
            EventPayload::from_str("{}")?,
        );
        let stream = EventStream::new(vec![event.clone()])?;
        assert_eq!(stream.id(), event.stream_id());
        assert_eq!(stream.seq(), event.stream_seq());
        assert_eq!(stream.events(), vec![event]);
        Ok(())
    }

    #[test]
    fn new_test_invalid_event_stream_id() -> anyhow::Result<()> {
        let event1 = Event::new(
            EventId::generate(),
            EventType::from_str("created")?,
            EventStreamId::generate(),
            EventStreamSeq::from(1_u32),
            EventAt::now(),
            EventPayload::from_str("{}")?,
        );
        let event2 = Event::new(
            EventId::generate(),
            EventType::from_str("updated")?,
            EventStreamId::generate(),
            EventStreamSeq::from(2_u32),
            EventAt::now(),
            EventPayload::from_str("{}")?,
        );
        assert!(EventStream::new(vec![event1, event2]).is_err());
        Ok(())
    }

    #[test]
    fn new_test_invalid_event_stream_seq() -> anyhow::Result<()> {
        let stream_id = EventStreamId::generate();
        let event1 = Event::new(
            EventId::generate(),
            EventType::from_str("created")?,
            stream_id,
            EventStreamSeq::from(1_u32),
            EventAt::now(),
            EventPayload::from_str("{}")?,
        );
        let event2 = Event::new(
            EventId::generate(),
            EventType::from_str("updated")?,
            stream_id,
            EventStreamSeq::from(1_u32),
            EventAt::now(),
            EventPayload::from_str("{}")?,
        );
        assert!(EventStream::new(vec![event1, event2]).is_err());
        Ok(())
    }

    #[test]
    fn new_test_ok_two_events() -> anyhow::Result<()> {
        let stream_id = EventStreamId::generate();
        let event1 = Event::new(
            EventId::generate(),
            EventType::from_str("created")?,
            stream_id,
            EventStreamSeq::from(1_u32),
            EventAt::now(),
            EventPayload::from_str("{}")?,
        );
        let event2 = Event::new(
            EventId::generate(),
            EventType::from_str("updated")?,
            stream_id,
            EventStreamSeq::from(2_u32),
            EventAt::now(),
            EventPayload::from_str("{}")?,
        );
        let stream = EventStream::new(vec![event1.clone(), event2.clone()])?;
        assert_eq!(stream.id(), event1.stream_id());
        assert_eq!(stream.seq(), event2.stream_seq());
        assert_eq!(stream.events(), vec![event1, event2]);
        Ok(())
    }

    #[test]
    fn push_test() -> anyhow::Result<()> {
        let mut stream = EventStream::generate(
            EventType::from_str("created")?,
            EventPayload::from_str("{}")?,
        );
        stream.push(
            EventType::from_str("updated")?,
            EventPayload::from_str(r#"{"key":123}"#)?,
        )?;
        assert_eq!(stream.seq(), EventStreamSeq::from(2_u32));
        Ok(())
    }

    #[test]
    fn push_event_test() -> anyhow::Result<()> {
        let mut stream = EventStream::generate(
            EventType::from_str("created")?,
            EventPayload::from_str("{}")?,
        );
        stream.push_event(Event::new(
            EventId::generate(),
            EventType::from_str("updated")?,
            stream.id(),
            EventStreamSeq::from(2_u32),
            EventAt::now(),
            EventPayload::from_str(r#"{"key":123}"#)?,
        ))?;
        assert_eq!(stream.seq(), EventStreamSeq::from(2_u32));
        Ok(())
    }

    #[test]
    fn push_event_test_invalid_event_stream_id() -> anyhow::Result<()> {
        let mut stream = EventStream::generate(
            EventType::from_str("created")?,
            EventPayload::from_str("{}")?,
        );
        assert!(stream
            .push_event(Event::new(
                EventId::generate(),
                EventType::from_str("updated")?,
                EventStreamId::generate(),
                EventStreamSeq::from(2_u32),
                EventAt::now(),
                EventPayload::from_str(r#"{"key":123}"#)?,
            ))
            .is_err());
        Ok(())
    }

    #[test]
    fn push_event_test_invalid_event_stream_seq() -> anyhow::Result<()> {
        let mut stream = EventStream::generate(
            EventType::from_str("created")?,
            EventPayload::from_str("{}")?,
        );
        assert!(stream
            .push_event(Event::new(
                EventId::generate(),
                EventType::from_str("updated")?,
                stream.id(),
                EventStreamSeq::from(1_u32),
                EventAt::now(),
                EventPayload::from_str(r#"{"key":123}"#)?,
            ))
            .is_err());
        Ok(())
    }
}
