use crate::events::UserCreated;
use crate::events::UserEvent;
use crate::events::UserUpdated;

#[derive(Debug, thiserror::Error)]
pub enum QueryUserError {
    #[error("invalid update event: {0:?}")]
    ApplyWithInitialEvent(UserEvent),
    #[error("empty event stream")]
    EmptyEventStream,
    #[error("recreate with non initial event: {0:?}")]
    RecreateWithNonInitialEvent(UserEvent),
}

#[derive(Clone)]
pub struct QueryUser {
    pub id: String,
    pub name: String,
    pub version: u32,
}

impl QueryUser {
    pub fn from_events<I>(events: I) -> Result<QueryUser, QueryUserError>
    where
        I: IntoIterator<Item = UserEvent>,
    {
        let mut iter = events.into_iter();
        let initial_event = iter.next().ok_or(QueryUserError::EmptyEventStream)?;
        let mut user = QueryUser::recreate(initial_event)?;
        for event in iter {
            user.apply(event)?;
        }
        Ok(user)
    }

    fn recreate(event: UserEvent) -> Result<QueryUser, QueryUserError> {
        match event {
            UserEvent::Created(UserCreated {
                at: _,
                id: _,
                name,
                user_id,
                version,
            }) => Ok(QueryUser {
                id: user_id,
                name,
                version,
            }),
            UserEvent::Updated(_) => Err(QueryUserError::RecreateWithNonInitialEvent(event)),
        }
    }

    fn apply(&mut self, event: UserEvent) -> Result<(), QueryUserError> {
        match event {
            UserEvent::Created(_) => Err(QueryUserError::ApplyWithInitialEvent(event)),
            UserEvent::Updated(UserUpdated {
                at: _,
                id: _,
                name,
                user_id,
                version,
            }) => {
                assert_eq!(self.id, user_id);
                self.name = name;
                assert!(self.version < version);
                self.version = version;
                Ok(())
            }
        }
    }
}
