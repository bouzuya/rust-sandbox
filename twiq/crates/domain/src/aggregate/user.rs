mod event;
mod value;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
};
use time::OffsetDateTime;

use self::{
    event::{Event, UserCreated, UserFetchRequested, UserUpdated},
    value::{twitter_user_id::TwitterUserId, user_id::UserId},
};

#[derive(Debug, thiserror::Error)]
#[error("error")]
pub struct Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct User {
    events: Vec<Event>,
}

impl User {
    pub fn create(twitter_user_id: TwitterUserId) -> Result<Self> {
        let id = EventId::generate();
        let at = OffsetDateTime::now_utc();
        // user_id = event_stream_id
        let user_id = UserId::generate();
        let stream_id = EventStreamId::try_from(u128::from(user_id)).map_err(|_| {
            // TODO: error handling
            Error
        })?;
        let stream_seq = EventStreamSeq::from(1);
        Ok(Self {
            events: vec![Event::Created(UserCreated::new(
                id,
                at,
                stream_id,
                stream_seq,
                twitter_user_id,
            ))],
        })
    }

    pub fn fetch_request(&mut self) -> Result<()> {
        // TODO
        self.events.push(Event::FetchRequested(UserFetchRequested));
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        // TODO
        self.events.push(Event::Updated(UserUpdated));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_test() -> anyhow::Result<()> {
        let twitter_user_id = "bouzuya".parse()?;
        let user = User::create(twitter_user_id)?;
        assert!(matches!(user.events[0], Event::Created(_)));
        // TODO: check twitter_user_id
        Ok(())
    }

    #[test]
    fn fetch_request_test() {
        // TODO
    }

    #[test]
    fn update_test() {
        // TODO
    }
}
