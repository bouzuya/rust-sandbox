mod event;
pub mod value;

use event_store_core::EventStream;

use crate::{
    event::EventType,
    value::{At, TwitterUserId, UserId, UserRequestId},
};

pub use self::event::{Event, UserRequestCreated, UserRequestFinished, UserRequestStarted};
use self::value::user_response::UserResponse;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRequest {
    event_stream: EventStream,
    id: UserRequestId,
    user_id: UserId,
}

impl UserRequest {
    pub fn create(
        id: UserRequestId,
        twitter_user_id: TwitterUserId,
        user_id: UserId,
    ) -> Result<Self> {
        let user_request_created = UserRequestCreated::new(At::now(), twitter_user_id, user_id, id);
        let event_stream =
            EventStream::generate(UserRequestCreated::r#type(), user_request_created);
        Ok(Self {
            event_stream,
            id,
            user_id,
        })
    }

    pub fn finish(&self, user_response: UserResponse) -> Result<Self> {
        if !self
            .event_stream
            .events()
            .last()
            .map(|event| {
                EventType::try_from(event.r#type().clone()).unwrap() == UserRequestStarted::r#type()
            })
            .unwrap_or_default()
        {
            return Err(Error::Unknown(
                "user_request status is not started".to_owned(),
            ));
        }
        let user_request_finished =
            UserRequestFinished::new(At::now(), self.user_id, self.id, user_response);
        let mut cloned = self.clone();
        cloned
            .event_stream
            .push2(UserRequestFinished::r#type(), user_request_finished)
            .unwrap();
        Ok(cloned)
    }

    pub fn start(&self) -> Result<Self> {
        if !self
            .event_stream
            .events()
            .last()
            .map(|event| {
                EventType::try_from(event.r#type().clone()).unwrap() == UserRequestCreated::r#type()
            })
            .unwrap_or_default()
        {
            return Err(Error::Unknown(
                "user_request status is not created".to_owned(),
            ));
        }
        let user_request_started = UserRequestStarted::new(At::now(), self.id);
        let mut cloned = self.clone();
        cloned
            .event_stream
            .push2(UserRequestStarted::r#type(), user_request_started)
            .unwrap();
        Ok(cloned)
    }

    pub fn id(&self) -> UserRequestId {
        self.id
    }
}

impl From<UserRequest> for EventStream {
    fn from(user: UserRequest) -> Self {
        user.event_stream
    }
}

impl TryFrom<EventStream> for UserRequest {
    type Error = Error;

    fn try_from(event_stream: EventStream) -> Result<Self, Self::Error> {
        use crate::Event as DomainEvent;
        use event_store_core::Event as RawEvent;
        let try_from_raw_event = |raw_event: RawEvent| -> Result<Event, Self::Error> {
            let domain_event =
                DomainEvent::try_from(raw_event).map_err(|e| Error::Unknown(e.to_string()))?;
            let aggregate_event =
                Event::try_from(domain_event).map_err(|e| Error::Unknown(e.to_string()))?;
            Ok(aggregate_event)
        };
        let raw_events = event_stream.events();
        let mut user_request = match try_from_raw_event(raw_events[0].clone())? {
            Event::Created(event) => UserRequest {
                event_stream: event_stream.clone(),
                id: event.user_request_id(),
                user_id: event.user_id(),
            },
            _ => {
                return Err(Error::Unknown(
                    "first event is not created event".to_owned(),
                ))
            }
        };
        for raw_event in raw_events.into_iter().skip(1) {
            let user_request_event = try_from_raw_event(raw_event)?;
            user_request = match user_request_event {
                Event::Created(_) => return Err(Error::Unknown("invalid event stream".to_owned())),
                Event::Started(_) => UserRequest {
                    event_stream: event_stream.clone(),
                    ..user_request
                },
                Event::Finished(_) => UserRequest {
                    event_stream: event_stream.clone(),
                    ..user_request
                },
            };
        }
        Ok(user_request)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn id_test() -> anyhow::Result<()> {
        let id = UserRequestId::generate();
        let twitter_user_id = TwitterUserId::from_str("125962981")?;
        let user_id = UserId::generate();
        let user_request = UserRequest::create(id, twitter_user_id, user_id)?;
        assert_eq!(user_request.id(), id);
        Ok(())
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        use event_store_core::EventType as RawEventType;
        let id = UserRequestId::generate();
        let twitter_user_id = TwitterUserId::from_str("bouzuya")?;
        let user_id = UserId::generate();
        let user_request = UserRequest::create(id, twitter_user_id, user_id)?;
        assert_eq!(
            user_request.event_stream.events()[0].r#type(),
            &RawEventType::from(UserRequestCreated::r#type())
        );
        let started = user_request.start()?;
        assert_eq!(
            started.event_stream.events()[1].r#type(),
            &RawEventType::from(UserRequestStarted::r#type()),
        );
        let finished = started.finish(UserResponse::new(200, "{}".to_owned()))?;
        assert_eq!(
            finished.event_stream.events()[2].r#type(),
            &RawEventType::from(UserRequestFinished::r#type()),
        );
        Ok(())
    }
}
