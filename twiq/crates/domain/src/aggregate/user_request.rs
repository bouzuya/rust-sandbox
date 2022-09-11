mod event;

use event_store_core::{
    event_id::EventId, event_stream_id::EventStreamId, event_stream_seq::EventStreamSeq,
};

use crate::value::{At, TwitterUserId, UserId, UserRequestId, Version};

use self::event::{Event, UserRequestCreated};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct UserRequest {
    events: Vec<Event>,
    id: UserRequestId,
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
            version: Version::from(stream_seq),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn create_test() -> anyhow::Result<()> {
        let id = UserRequestId::generate();
        let twitter_user_id = TwitterUserId::from_str("bouzuya")?;
        let user_id = UserId::generate();
        let user_request = UserRequest::create(id, twitter_user_id, user_id)?;
        assert!(matches!(user_request.events[0], Event::Created(_)));
        Ok(())
    }
}
