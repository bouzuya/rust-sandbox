use std::str::FromStr;

use event_store_core::{Event as RawEvent, EventType as RawEventType};

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventType {
    UserCreated,
    UserRequested,
    UserUpdated,
    UserRequestCreated,
    UserRequestStarted,
    UserRequestFinished,
}

impl From<EventType> for RawEventType {
    fn from(event_type: EventType) -> Self {
        use EventType::*;
        RawEventType::from_str(match event_type {
            UserCreated => "user_created",
            UserRequested => "user_requested",
            UserUpdated => "user_updated",
            UserRequestCreated => "user_request_created",
            UserRequestStarted => "user_request_started",
            UserRequestFinished => "user_request_finished",
        })
        .expect("event_type")
    }
}

impl TryFrom<RawEventType> for EventType {
    type Error = Error;

    fn try_from(value: RawEventType) -> Result<Self, Self::Error> {
        use EventType::*;
        let event_type = match value.as_str() {
            "user_created" => UserCreated,
            "user_requested" => UserRequested,
            "user_updated" => UserUpdated,
            "user_request_created" => UserRequestCreated,
            "user_request_started" => UserRequestStarted,
            "user_request_finished" => UserRequestFinished,
            _ => return Err(Error::Unknown("unknown event_type".to_owned())),
        };
        Ok(event_type)
    }
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
        let s = String::from(raw_event.data().clone());
        let event: Event =
            serde_json::from_str(s.as_str()).map_err(|e| Error::Unknown(e.to_string()))?;
        assert_eq!(raw_event, RawEvent::from(event.clone()));
        Ok(event)
    }
}

// TODO: test serde

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_type_test() -> anyhow::Result<()> {
        let types = vec![
            (EventType::UserCreated, "user_created"),
            (EventType::UserRequested, "user_requested"),
            (EventType::UserUpdated, "user_updated"),
            (EventType::UserRequestCreated, "user_request_created"),
            (EventType::UserRequestStarted, "user_request_started"),
            (EventType::UserRequestFinished, "user_request_finished"),
        ];
        for (t, s) in types {
            assert_eq!(EventType::try_from(RawEventType::from(t))?, t);
            assert_eq!(RawEventType::from_str(s)?.as_str(), s);
        }
        Ok(())
    }
}
