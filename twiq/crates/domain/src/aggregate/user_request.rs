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
            EventStream::generate2(UserRequestCreated::r#type(), user_request_created);
        Ok(Self {
            event_stream,
            id,
            user_id,
        })
    }

    pub fn finish(&mut self, user_response: UserResponse) -> Result<()> {
        if !self
            .event_stream
            .events()
            .last()
            .map(|event| {
                EventType::try_from(*event.r#type()).unwrap() == UserRequestStarted::r#type()
            })
            .unwrap_or_default()
        {
            return Err(Error::Unknown(
                "user_request status is not started".to_owned(),
            ));
        }
        let user_request_finished =
            UserRequestFinished::new(At::now(), self.user_id, self.id, user_response);
        self.event_stream
            .push2(UserRequestFinished::r#type(), user_request_finished);
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        if !self
            .event_stream
            .events()
            .last()
            .map(|event| {
                EventType::try_from(*event.r#type()).unwrap() == UserRequestCreated::r#type()
            })
            .unwrap_or_default()
        {
            return Err(Error::Unknown(
                "user_request status is not created".to_owned(),
            ));
        }
        let user_request_started = UserRequestStarted::new(At::now(), self.id);
        self.event_stream
            .push2(UserRequestStarted::r#type(), user_request_started);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        use crate::Event as DomainEvent;
        let id = UserRequestId::generate();
        let twitter_user_id = TwitterUserId::from_str("bouzuya")?;
        let user_id = UserId::generate();
        let mut user_request = UserRequest::create(id, twitter_user_id, user_id)?;
        assert!(matches!(
            DomainEvent::try_from(user_request.event_stream.events()[0])?,
            DomainEvent::UserRequestCreated(_)
        ));
        user_request.start()?;
        assert!(matches!(
            DomainEvent::try_from(user_request.event_stream.events()[1])?,
            DomainEvent::UserRequestStarted(_)
        ));
        user_request.finish(UserResponse::new(200, "{}".to_owned()))?;
        assert!(matches!(
            DomainEvent::try_from(user_request.event_stream.events()[2])?,
            DomainEvent::UserRequestFinished(_)
        ));
        Ok(())
    }
}
