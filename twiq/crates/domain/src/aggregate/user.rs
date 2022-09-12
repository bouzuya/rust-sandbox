mod event;
mod value;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
};

pub use crate::value::{At, TwitterUserId, UserId, UserRequestId, Version};

use self::{
    event::{Event, UserCreated, UserRequested, UserUpdated},
    value::twitter_user_name::TwitterUserName,
};

#[derive(Debug, thiserror::Error)]
#[error("error")]
pub struct Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

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

    pub fn request(&mut self, at: At) -> Result<()> {
        if let Some(fetch_requested_at) = self.fetch_requested_at {
            if at <= fetch_requested_at.plus_1day() {
                // TODo: error handling
                return Err(Error);
            }
        }
        let user_id = self.user_id;
        let stream_seq = EventStreamSeq::from(self.version).next().map_err(|_| {
            // TODO: error handling
            Error
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
                return Err(Error);
            }
        }
        let stream_seq = EventStreamSeq::from(self.version).next().map_err(|_| {
            // TODO: error handling
            Error
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

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
