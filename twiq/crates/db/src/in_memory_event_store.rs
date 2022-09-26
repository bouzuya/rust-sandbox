use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use event_store_core::{
    event_store::{Error, EventStore, Result},
    Event, EventId, EventStream, EventStreamId, EventStreamSeq,
};

#[derive(Debug, Default)]
pub struct InMemoryEventStore {
    events: Arc<Mutex<Vec<Event>>>,
    event_ids: Arc<Mutex<HashMap<EventId, usize>>>,
    event_streams: Arc<Mutex<HashMap<EventStreamId, EventStreamSeq>>>,
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn find_event(&self, event_id: EventId) -> Result<Option<Event>> {
        let event_ids = self.event_ids.lock().unwrap();
        let events = self.events.lock().unwrap();
        let event = event_ids
            .get(&event_id)
            .and_then(|&index| events.get(index))
            .cloned();
        Ok(event)
    }

    async fn find_event_ids(&self, after: Option<EventId>) -> Result<Vec<EventId>> {
        self.find_events(after)
            .await
            .map(|events| events.into_iter().map(|event| event.id()).collect())
    }

    async fn find_event_stream(
        &self,
        event_stream_id: EventStreamId,
    ) -> Result<Option<EventStream>> {
        let events = self.events.lock().unwrap();
        let events = events
            .iter()
            .filter(|event| event.stream_id() == event_stream_id)
            .cloned()
            .collect::<Vec<Event>>();
        if events.is_empty() {
            Ok(None)
        } else {
            Ok(EventStream::new(events).map(Some).unwrap())
        }
    }

    async fn find_events(&self, after: Option<EventId>) -> Result<Vec<Event>> {
        let event_ids = self.event_ids.lock().unwrap();
        let events = self.events.lock().unwrap();
        let index = after
            .and_then(|event_id| event_ids.get(&event_id))
            .copied()
            .unwrap_or_default();
        Ok(events[index..].to_vec())
    }

    // = store_event_stream
    async fn store(
        &self,
        current: Option<EventStreamSeq>,
        event_stream: EventStream,
    ) -> Result<()> {
        let mut event_ids = self.event_ids.lock().unwrap();
        let mut event_streams = self.event_streams.lock().unwrap();
        let mut events = self.events.lock().unwrap();

        let event_stream_id = event_stream.id();
        let event_stream_seq = event_stream.seq();
        let stored_event_stream_seq = event_streams.get(&event_stream_id);
        match (current, stored_event_stream_seq.copied()) {
            (None, None) => {
                event_streams.insert(event_stream_id, event_stream_seq);
            }
            (None, Some(_)) => return Err(Error::Unknown("already exists".to_owned())),
            (Some(_), None) => return Err(Error::Unknown("not found".to_owned())),
            (Some(expected), Some(actual)) => {
                if expected != actual {
                    return Err(Error::Unknown("does not match".to_owned()));
                }

                event_streams.insert(event_stream_id, event_stream_seq);
            }
        }

        for event in event_stream.events() {
            let index = events.len();
            if event_ids.insert(event.id(), index).is_none() {
                events.push(event);
            }
        }

        Ok(())
    }
}
