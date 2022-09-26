mod event;
mod value;

use event_store_core::{
    event_id::EventId, event_stream::EventStream, event_stream_id::EventStreamId,
    event_stream_seq::EventStreamSeq,
};

pub use crate::value::{At, TwitterUserId, UserId, UserRequestId, Version};

pub use self::{
    event::{Event, UserCreated, UserRequested, UserUpdated},
    value::twitter_user_name::TwitterUserName,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("error")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    events: Vec<Event>,
    fetch_requested_at: Option<At>,
    twitter_user_id: TwitterUserId,
    updated_at: Option<At>,
    user_id: UserId,
    version: Version,
}

impl User {
    pub fn create(twitter_user_id: TwitterUserId) -> Result<Self> {
        let user_id = UserId::generate();
        let stream_seq = EventStreamSeq::from(1);
        Ok(Self {
            events: vec![Event::Created(UserCreated::new(
                EventId::generate(),
                At::now(),
                EventStreamId::from(user_id),
                stream_seq,
                twitter_user_id.clone(),
            ))],
            fetch_requested_at: None,
            twitter_user_id,
            updated_at: None,
            user_id,
            version: Version::from(stream_seq),
        })
    }

    pub fn id(&self) -> UserId {
        self.user_id
    }

    pub fn request(&mut self, at: At) -> Result<()> {
        if let Some(fetch_requested_at) = self.fetch_requested_at {
            if at <= fetch_requested_at.plus_1day() {
                // TODo: error handling
                return Err(Error::Unknown("".to_owned()));
            }
        }
        let user_id = self.user_id;
        let stream_seq = EventStreamSeq::from(self.version).next().map_err(|e| {
            // TODO: error handling
            Error::Unknown(e.to_string())
        })?;
        let user_request_id = UserRequestId::generate();
        self.events.push(Event::Requested(UserRequested::new(
            EventId::generate(),
            at,
            EventStreamId::from(user_id),
            stream_seq,
            self.twitter_user_id.clone(),
            user_request_id,
        )));
        self.fetch_requested_at = Some(at);
        self.version = Version::from(stream_seq);
        Ok(())
    }

    pub fn update(&mut self, name: TwitterUserName, at: At) -> Result<()> {
        if let Some(updated_at) = self.updated_at {
            if at <= updated_at {
                // TODo: error handling
                return Err(Error::Unknown("".to_owned()));
            }
        }
        let stream_seq = EventStreamSeq::from(self.version).next().map_err(|e| {
            // TODO: error handling
            Error::Unknown(e.to_string())
        })?;
        self.events.push(Event::Updated(UserUpdated::new(
            EventId::generate(),
            at,
            EventStreamId::from(self.user_id),
            stream_seq,
            self.twitter_user_id.clone(),
            name,
        )));
        self.updated_at = Some(at);
        self.version = Version::from(stream_seq);
        Ok(())
    }
}

impl From<User> for EventStream {
    fn from(user: User) -> Self {
        use crate::Event as DomainEvent;
        use event_store_core::Event as RawEvent;
        let mut events = vec![];
        for user_event in user.events.iter().cloned() {
            events.push(RawEvent::from(DomainEvent::from(user_event)));
        }
        EventStream::new(events).expect("event_stream")
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
                events: vec![Event::from(event.clone())],
                fetch_requested_at: None,
                twitter_user_id: event.twitter_user_id(),
                updated_at: None,
                user_id: event.user_id(),
                version: Version::from(EventStreamSeq::from(1_u32)),
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
                    events: {
                        user.events.push(Event::from(e.clone()));
                        user.events
                    },
                    fetch_requested_at: Some(e.at()),
                    version: Version::from(e.stream_seq()),
                    ..user
                },
                Event::Updated(e) => User {
                    events: {
                        user.events.push(Event::from(e.clone()));
                        user.events
                    },
                    version: Version::from(e.stream_seq()),
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

    #[test]
    fn event_stream_conversion_test() -> anyhow::Result<()> {
        use crate::Event as DomainEvent;
        use event_store_core::Event as RawEvent;
        let event_stream_id = EventStreamId::generate();
        let twitter_user_id = TwitterUserId::from_str("123")?;
        let event_stream = EventStream::new(vec![
            RawEvent::from(DomainEvent::from(UserCreated::new(
                EventId::generate(),
                At::now(),
                event_stream_id,
                EventStreamSeq::from(1_u32),
                twitter_user_id.clone(),
            ))),
            RawEvent::from(DomainEvent::from(UserRequested::new(
                EventId::generate(),
                At::now(),
                event_stream_id,
                EventStreamSeq::from(2_u32),
                twitter_user_id,
                UserRequestId::generate(),
            ))),
        ])?;
        let user = User::try_from(event_stream.clone())?;
        assert_eq!(EventStream::from(user), event_stream);
        Ok(())
    }

    #[test]
    fn create_test() -> anyhow::Result<()> {
        let twitter_user_id = "bouzuya".parse::<TwitterUserId>()?;
        let user = User::create(twitter_user_id)?;
        assert!(matches!(user.events[0], Event::Created(_)));
        // TODO: check twitter_user_id
        Ok(())
    }

    #[test]
    fn request_test() -> anyhow::Result<()> {
        let twitter_user_id = "123".parse::<TwitterUserId>()?;
        let mut user = User::create(twitter_user_id)?;
        let at = At::now();
        user.request(at)?;
        assert!(matches!(user.events[1], Event::Requested(_)));
        let at = At::now();
        assert!(user.request(at).is_err());
        assert_eq!(user.events.len(), 2);
        Ok(())
    }

    #[test]
    fn update_test() -> anyhow::Result<()> {
        let twitter_user_id = "123".parse::<TwitterUserId>()?;
        let mut user = User::create(twitter_user_id)?;
        let at = At::now();
        let name = TwitterUserName::from_str("bouzuya")?;
        user.update(name, at)?;
        assert!(matches!(user.events[1], Event::Updated(_)));
        Ok(())
    }
}
