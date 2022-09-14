use event_store_core::Event as RawEvent;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Event {
    User(crate::aggregate::user::Event),
    UserRequest(crate::aggregate::user_request::Event),
}

impl From<Event> for RawEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::User(e) => RawEvent::from(e),
            Event::UserRequest(e) => RawEvent::from(e),
        }
    }
}

impl TryFrom<RawEvent> for Event {
    type Error = Error;

    fn try_from(raw_event: RawEvent) -> Result<Self, Self::Error> {
        let s = String::from(raw_event.clone().data());
        let event: Event =
            serde_json::from_str(s.as_str()).map_err(|e| Error::Unknown(e.to_string()))?;
        assert_eq!(raw_event, RawEvent::from(event.clone()));
        Ok(event)
    }
}

// TODO: test serde
