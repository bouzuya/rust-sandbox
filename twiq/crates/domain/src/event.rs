use std::{fmt::Display, str::FromStr};

use event_store_core::{Event as RawEvent, EventPayload, EventType as RawEventType};

use crate::aggregate::{
    user::{UserCreated, UserRequested, UserUpdated},
    user_request::{UserRequestCreated, UserRequestFinished, UserRequestStarted},
};

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventStreamType {
    User,
    UserRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum EventType {
    UserCreated,
    UserRequested,
    UserUpdated,
    UserRequestCreated,
    UserRequestStarted,
    UserRequestFinished,
}

impl EventType {
    pub fn event_stream_type(&self) -> EventStreamType {
        use EventStreamType::*;
        use EventType::*;
        match self {
            UserCreated => User,
            UserRequested => User,
            UserUpdated => User,
            UserRequestCreated => UserRequest,
            UserRequestStarted => UserRequest,
            UserRequestFinished => UserRequest,
        }
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", RawEventType::from(*self))
    }
}

impl FromStr for EventType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_event_type =
            RawEventType::from_str(s).map_err(|e| Error::Unknown(e.to_string()))?;
        EventType::try_from(raw_event_type)
    }
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    UserCreated(crate::aggregate::user::UserCreated),
    UserRequested(crate::aggregate::user::UserRequested),
    UserUpdated(crate::aggregate::user::UserUpdated),
    UserRequestCreated(crate::aggregate::user_request::UserRequestCreated),
    UserRequestStarted(crate::aggregate::user_request::UserRequestStarted),
    UserRequestFinished(crate::aggregate::user_request::UserRequestFinished),
}

macro_rules! impl_from_and_try_from {
    ($constructor: path, $ty: ty) => {
        impl From<$ty> for Event {
            fn from(value: $ty) -> Self {
                $constructor(value)
            }
        }

        impl TryFrom<Event> for $ty {
            type Error = Error;

            fn try_from(value: Event) -> Result<Self, Self::Error> {
                if let $constructor(value) = value {
                    Ok(value)
                } else {
                    Err(Error::Unknown("try from failed".to_owned()))
                }
            }
        }
    };
}

impl_from_and_try_from!(Event::UserCreated, UserCreated);
impl_from_and_try_from!(Event::UserRequested, UserRequested);
impl_from_and_try_from!(Event::UserUpdated, UserUpdated);
impl_from_and_try_from!(Event::UserRequestCreated, UserRequestCreated);
impl_from_and_try_from!(Event::UserRequestStarted, UserRequestStarted);
impl_from_and_try_from!(Event::UserRequestFinished, UserRequestFinished);

impl From<Event> for EventPayload {
    fn from(event: Event) -> Self {
        match event {
            Event::UserCreated(e) => EventPayload::from(e),
            Event::UserRequested(e) => EventPayload::from(e),
            Event::UserUpdated(e) => EventPayload::from(e),
            Event::UserRequestCreated(e) => EventPayload::from(e),
            Event::UserRequestStarted(e) => EventPayload::from(e),
            Event::UserRequestFinished(e) => EventPayload::from(e),
        }
    }
}

impl TryFrom<RawEvent> for Event {
    type Error = Error;

    fn try_from(raw_event: RawEvent) -> Result<Self, Self::Error> {
        let event_type = EventType::try_from(raw_event.r#type().clone())?;
        let event = match event_type {
            EventType::UserCreated => Event::from(
                UserCreated::try_from(raw_event).map_err(|e| Error::Unknown(e.to_string()))?,
            ),
            EventType::UserRequested => Event::from(
                UserRequested::try_from(raw_event).map_err(|e| Error::Unknown(e.to_string()))?,
            ),
            EventType::UserUpdated => Event::from(
                UserUpdated::try_from(raw_event).map_err(|e| Error::Unknown(e.to_string()))?,
            ),
            EventType::UserRequestCreated => Event::from(
                UserRequestCreated::try_from(raw_event)
                    .map_err(|e| Error::Unknown(e.to_string()))?,
            ),
            EventType::UserRequestStarted => Event::from(
                UserRequestStarted::try_from(raw_event)
                    .map_err(|e| Error::Unknown(e.to_string()))?,
            ),
            EventType::UserRequestFinished => Event::from(
                UserRequestFinished::try_from(raw_event)
                    .map_err(|e| Error::Unknown(e.to_string()))?,
            ),
        };
        Ok(event)
    }
}

impl From<crate::aggregate::user::Event> for Event {
    fn from(event: crate::aggregate::user::Event) -> Self {
        match event {
            crate::aggregate::user::Event::Created(e) => Event::from(e),
            crate::aggregate::user::Event::Requested(e) => Event::from(e),
            crate::aggregate::user::Event::Updated(e) => Event::from(e),
        }
    }
}

impl TryFrom<Event> for crate::aggregate::user::Event {
    type Error = Error;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        Ok(match value {
            Event::UserCreated(e) => crate::aggregate::user::Event::from(e),
            Event::UserRequested(e) => crate::aggregate::user::Event::from(e),
            Event::UserUpdated(e) => crate::aggregate::user::Event::from(e),
            _ => return Err(Error::Unknown("invalid event type".to_owned())),
        })
    }
}

impl From<crate::aggregate::user_request::Event> for Event {
    fn from(event: crate::aggregate::user_request::Event) -> Self {
        match event {
            crate::aggregate::user_request::Event::Created(e) => Event::from(e),
            crate::aggregate::user_request::Event::Started(e) => Event::from(e),
            crate::aggregate::user_request::Event::Finished(e) => Event::from(e),
        }
    }
}

impl TryFrom<Event> for crate::aggregate::user_request::Event {
    type Error = Error;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        Ok(match value {
            Event::UserRequestCreated(e) => crate::aggregate::user_request::Event::from(e),
            Event::UserRequestStarted(e) => crate::aggregate::user_request::Event::from(e),
            Event::UserRequestFinished(e) => crate::aggregate::user_request::Event::from(e),
            _ => return Err(Error::Unknown("invalid event type".to_owned())),
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use core::fmt::Debug;

    pub(crate) fn serde_test<T>(o: T, s: &str) -> anyhow::Result<()>
    where
        T: Debug + Eq + serde::de::DeserializeOwned + serde::Serialize,
    {
        let deserialized: T = serde_json::from_str(s)?;
        assert_eq!(deserialized, o);
        assert_eq!(serde_json::to_string_pretty(&o)?, s);
        Ok(())
    }

    #[test]
    fn event_type_test() -> anyhow::Result<()> {
        use EventStreamType::*;
        use EventType::*;
        let types = vec![
            (UserCreated, "user_created", User),
            (UserRequested, "user_requested", User),
            (UserUpdated, "user_updated", User),
            (UserRequestCreated, "user_request_created", UserRequest),
            (UserRequestStarted, "user_request_started", UserRequest),
            (UserRequestFinished, "user_request_finished", UserRequest),
        ];
        for (t, s, st) in types {
            assert_eq!(EventType::try_from(RawEventType::from(t))?, t);
            assert_eq!(RawEventType::from_str(s)?.as_str(), s);
            assert_eq!(t.event_stream_type(), st);
        }

        let deserialized: EventType =
            serde_json::from_str(r#"{"type":"user_created","ignore":"unused"}"#)?;
        assert_eq!(deserialized, UserCreated);

        assert_eq!(
            EventType::from_str("user_created")?.to_string(),
            "user_created"
        );
        Ok(())
    }
}
