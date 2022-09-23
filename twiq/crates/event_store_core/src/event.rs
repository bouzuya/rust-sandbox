use crate::{
    event_data::EventData, event_id::EventId, event_stream_id::EventStreamId,
    event_stream_seq::EventStreamSeq, event_type::EventType,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    id: EventId,
    r#type: EventType,
    stream_id: EventStreamId,
    stream_seq: EventStreamSeq,
    data: EventData,
}

impl Event {
    pub fn new(
        id: EventId,
        r#type: EventType,
        stream_id: EventStreamId,
        stream_seq: EventStreamSeq,
        data: EventData,
    ) -> Self {
        Self {
            id,
            r#type,
            stream_id,
            stream_seq,
            data,
        }
    }

    pub fn id(&self) -> EventId {
        self.id
    }

    pub fn r#type(&self) -> &EventType {
        &self.r#type
    }

    pub fn stream_id(&self) -> EventStreamId {
        self.stream_id
    }

    pub fn stream_seq(&self) -> EventStreamSeq {
        self.stream_seq
    }

    pub fn data(&self) -> &EventData {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let id = EventId::generate();
        let r#type = EventType::try_from("created".to_owned())?;
        let stream_id = EventStreamId::generate();
        let stream_seq = EventStreamSeq::from(1_u32);
        let data = EventData::try_from(String::from("123"))?;
        let event = Event::new(id, r#type.clone(), stream_id, stream_seq, data.clone());
        assert_eq!(event.id(), id);
        assert_eq!(event.r#type(), &r#type);
        assert_eq!(event.stream_id(), stream_id);
        assert_eq!(event.stream_seq(), stream_seq);
        assert_eq!(event.data(), &data);
        Ok(())
    }
}
