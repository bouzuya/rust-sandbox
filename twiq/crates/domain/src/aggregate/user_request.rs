mod event;
pub mod value;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
};

use crate::value::{At, TwitterUserId, UserId, UserRequestId, Version};

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
    events: Vec<Event>,
    id: UserRequestId,
    user_id: UserId,
    version: Version,
}

impl UserRequest {
    pub fn create(
        id: UserRequestId,
        twitter_user_id: TwitterUserId,
        user_id: UserId,
    ) -> Result<Self> {
        let stream_seq = EventStreamSeq::from(1);
        Ok(Self {
            events: vec![Event::Created(UserRequestCreated::new(
                EventId::generate(),
                At::now(),
                EventStreamId::from(id),
                stream_seq,
                twitter_user_id,
                user_id,
            ))],
            id,
            user_id,
            version: Version::from(stream_seq),
        })
    }

    pub fn finish(&mut self, user_response: UserResponse) -> Result<()> {
        if !self
            .events
            .last()
            .map(|event| matches!(event, Event::Started(_)))
            .unwrap_or_default()
        {
            return Err(Error::Unknown(
                "user_request status is not started".to_owned(),
            ));
        }
        let stream_id = EventStreamId::from(self.id);
        // TODO: error handling
        let stream_seq = EventStreamSeq::from(self.version).next().unwrap();
        self.events.push(Event::Finished(UserRequestFinished::new(
            EventId::generate(),
            At::now(),
            stream_id,
            stream_seq,
            self.user_id,
            user_response,
        )));
        self.version = Version::from(stream_seq);
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        if !self
            .events
            .last()
            .map(|event| matches!(event, Event::Created(_)))
            .unwrap_or_default()
        {
            return Err(Error::Unknown(
                "user_request status is not created".to_owned(),
            ));
        }
        let stream_id = EventStreamId::from(self.id);
        // TODO: error handling
        let stream_seq = EventStreamSeq::from(self.version).next().unwrap();
        self.events.push(Event::Started(UserRequestStarted::new(
            EventId::generate(),
            At::now(),
            stream_id,
            stream_seq,
        )));
        self.version = Version::from(stream_seq);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let id = UserRequestId::generate();
        let twitter_user_id = TwitterUserId::from_str("bouzuya")?;
        let user_id = UserId::generate();
        let mut user_request = UserRequest::create(id, twitter_user_id, user_id)?;
        assert!(matches!(user_request.events[0], Event::Created(_)));
        user_request.start()?;
        assert!(matches!(user_request.events[1], Event::Started(_)));
        user_request.finish(UserResponse::new(200, "{}".to_owned()))?;
        assert!(matches!(user_request.events[2], Event::Finished(_)));
        Ok(())
    }
}
