mod event;
mod value;

use event_store_core::{event_stream::EventStream, EventPayload};

pub use crate::value::{At, TwitterUserId, UserId, UserRequestId, Version};

pub use self::{
    event::{Event, UserCreated, UserRequested, UserUpdated},
    value::twitter_user_name::TwitterUserName,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("already requested")]
    AlreadyRequested,
    #[error("error")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    event_stream: EventStream,
    fetch_requested_at: Option<At>,
    twitter_user_id: TwitterUserId,
    updated_at: Option<At>,
    user_id: UserId,
}

impl User {
    pub fn create(twitter_user_id: TwitterUserId) -> Result<Self> {
        let user_created = UserCreated::new(At::now(), twitter_user_id.clone(), UserId::generate());
        let user_id = user_created.user_id();
        let event_stream =
            EventStream::generate2(UserCreated::r#type(), EventPayload::from(user_created));
        Ok(Self {
            event_stream,
            fetch_requested_at: None,
            twitter_user_id,
            updated_at: None,
            user_id,
        })
    }

    pub fn id(&self) -> UserId {
        self.user_id
    }

    pub fn request(&mut self, at: At) -> Result<()> {
        if let Some(fetch_requested_at) = self.fetch_requested_at {
            if at <= fetch_requested_at.plus_1day() {
                return Err(Error::AlreadyRequested);
            }
        }
        let user_id = self.user_id;
        let user_request_id = UserRequestId::generate();
        self.event_stream
            .push2(
                UserRequested::r#type(),
                UserRequested::new(at, self.twitter_user_id.clone(), user_id, user_request_id),
            )
            .unwrap();
        self.fetch_requested_at = Some(at);
        Ok(())
    }

    pub fn twitter_user_id(&self) -> &TwitterUserId {
        &self.twitter_user_id
    }

    pub fn update(&mut self, name: TwitterUserName, at: At) -> Result<()> {
        if let Some(updated_at) = self.updated_at {
            if at <= updated_at {
                // TODO: error handling
                return Err(Error::Unknown("".to_owned()));
            }
        }
        self.event_stream
            .push2(
                UserUpdated::r#type(),
                UserUpdated::new(at, self.twitter_user_id.clone(), name, self.user_id),
            )
            .unwrap();
        self.updated_at = Some(at);
        Ok(())
    }
}

impl From<User> for EventStream {
    fn from(user: User) -> Self {
        user.event_stream
    }
}

impl TryFrom<EventStream> for User {
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
        let mut user = match try_from_raw_event(raw_events[0].clone())? {
            Event::Created(event) => User {
                event_stream: event_stream.clone(),
                fetch_requested_at: None,
                twitter_user_id: event.twitter_user_id().clone(),
                updated_at: None,
                user_id: event.user_id(),
            },
            _ => {
                return Err(Error::Unknown(
                    "first event is not created event".to_owned(),
                ))
            }
        };
        for raw_event in raw_events.into_iter().skip(1) {
            let user_event = try_from_raw_event(raw_event)?;
            user = match user_event {
                Event::Created(_) => return Err(Error::Unknown("invalid event stream".to_owned())),
                Event::Requested(e) => User {
                    event_stream: event_stream.clone(),
                    fetch_requested_at: Some(e.at()),
                    ..user
                },
                Event::Updated(e) => User {
                    event_stream: event_stream.clone(),
                    updated_at: Some(e.at()),
                    ..user
                },
            };
        }
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    // TODO: test twitter_user_id

    #[test]
    fn event_stream_conversion_test() -> anyhow::Result<()> {
        let twitter_user_id = TwitterUserId::from_str("123")?;
        let user_id = UserId::generate();
        let mut event_stream = EventStream::generate2(
            UserCreated::r#type(),
            UserCreated::new(At::now(), twitter_user_id.clone(), user_id),
        );
        event_stream.push2(
            UserRequested::r#type(),
            UserRequested::new(
                At::now(),
                twitter_user_id,
                user_id,
                UserRequestId::generate(),
            ),
        )?;
        let user = User::try_from(event_stream.clone())?;
        assert_eq!(EventStream::from(user), event_stream);
        Ok(())
    }

    #[test]
    fn create_test() -> anyhow::Result<()> {
        use event_store_core::EventType as RawEventType;
        let twitter_user_id = "bouzuya".parse::<TwitterUserId>()?;
        let user = User::create(twitter_user_id)?;
        assert_eq!(
            user.event_stream.events()[0].r#type(),
            &RawEventType::from(UserCreated::r#type())
        );
        // TODO: check twitter_user_id
        Ok(())
    }

    #[test]
    fn request_test() -> anyhow::Result<()> {
        use event_store_core::EventType as RawEventType;
        let twitter_user_id = "123".parse::<TwitterUserId>()?;
        let mut user = User::create(twitter_user_id)?;
        let at = At::now();
        user.request(at)?;
        assert_eq!(
            user.event_stream.events()[1].r#type(),
            &RawEventType::from(UserRequested::r#type()),
        );
        let at = At::now();
        assert!(user.request(at).is_err());
        assert_eq!(user.event_stream.events().len(), 2);
        Ok(())
    }

    #[test]
    fn update_test() -> anyhow::Result<()> {
        use event_store_core::EventType as RawEventType;
        let twitter_user_id = "123".parse::<TwitterUserId>()?;
        let mut user = User::create(twitter_user_id)?;
        let at = At::now();
        let name = TwitterUserName::from_str("bouzuya")?;
        user.update(name, at)?;
        assert_eq!(
            user.event_stream.events()[1].r#type(),
            &RawEventType::from(UserUpdated::r#type())
        );
        Ok(())
    }
}
