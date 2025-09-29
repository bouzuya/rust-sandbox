use crate::event::UserCreated;
use crate::event::UserEvent;
use crate::event::UserUpdated;
use crate::value_objects::EventId;
use crate::value_objects::UserId;
use crate::value_objects::UserIdError;
use crate::value_objects::UserName;
use crate::value_objects::UserNameError;
use crate::value_objects::Version;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("apply with initial event: {0:?}")]
    ApplyWithInitialEvent(UserEvent),
    #[error("empty event stream")]
    EmptyEventStream,
    #[error("invalid persisted user id: {0:?}")]
    InvalidPersistedUserId(#[source] UserIdError),
    #[error("invalid persisted user name: {0:?}")]
    InvalidPersistedUserName(#[source] UserNameError),
    #[error("recreate with non initial event: {0:?}")]
    RecreateWithNonInitialEvent(UserEvent),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    id: UserId,
    name: UserName,
    version: Version,
}

impl User {
    pub fn create(name: UserName) -> Result<(User, Vec<UserEvent>), UserError> {
        let event = UserEvent::Created(UserCreated {
            at: now(),
            id: String::from(EventId::new()),
            name: String::from(name.clone()),
            user_id: String::from(UserId::new()),
            version: u32::from(Version::new()),
        });
        let state = User::recreate(event.clone())?;
        Ok((state, vec![event]))
    }

    pub fn from_events<I>(events: I) -> Result<User, UserError>
    where
        I: IntoIterator<Item = UserEvent>,
    {
        let mut iter = events.into_iter();
        let initial_event = iter.next().ok_or(UserError::EmptyEventStream)?;
        let mut user = User::recreate(initial_event)?;
        for event in iter {
            user.apply(event)?;
        }
        Ok(user)
    }

    fn recreate(event: UserEvent) -> Result<User, UserError> {
        match event {
            UserEvent::Created(UserCreated {
                at: _,
                id: _,
                name,
                user_id,
                version,
            }) => Ok(User {
                id: UserId::try_from(user_id).map_err(UserError::InvalidPersistedUserId)?,
                name: UserName::try_from(name).map_err(UserError::InvalidPersistedUserName)?,
                version: Version::from(version),
            }),
            UserEvent::Updated(_) => Err(UserError::RecreateWithNonInitialEvent(event)),
        }
    }

    fn apply(&mut self, event: UserEvent) -> Result<(), UserError> {
        match event {
            UserEvent::Created(_) => Err(UserError::ApplyWithInitialEvent(event)),
            UserEvent::Updated(UserUpdated {
                at: _,
                id: _,
                name,
                user_id,
                version,
            }) => {
                assert_eq!(
                    self.id,
                    UserId::try_from(user_id).map_err(UserError::InvalidPersistedUserId)?
                );
                self.name =
                    UserName::try_from(name).map_err(UserError::InvalidPersistedUserName)?;
                let version = Version::from(version);
                assert!(self.version < version);
                self.version = version;
                Ok(())
            }
        }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn update(&self, name: UserName) -> Result<(Self, Vec<UserEvent>), UserError> {
        let event = UserEvent::Updated(UserUpdated {
            at: now(),
            id: String::from(EventId::new()),
            name: String::from(name),
            user_id: String::from(self.id.clone()),
            version: u32::from(self.version.next()),
        });

        let mut cloned = self.clone();
        cloned.apply(event.clone())?;

        Ok((cloned, vec![event]))
    }

    pub fn version(&self) -> Version {
        self.version
    }
}

fn now() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::event::UserCreated;
    use crate::event::UserEvent;
    use crate::event::UserUpdated;

    #[test]
    fn test_create() -> anyhow::Result<()> {
        let user_name = UserName::new_for_testing();
        let (user, events) = User::create(user_name.clone())?;
        assert_eq!(user.name, user_name);
        assert_eq!(user.version, Version::from(1));
        assert_eq!(events.len(), 1);
        match &events[0] {
            UserEvent::Created(UserCreated {
                at: _,
                id: _,
                name,
                user_id,
                version,
            }) => {
                assert_eq!(String::from(user.id.clone()), *user_id);
                assert_eq!(String::from(user.name.clone()), *name);
                assert_eq!(u32::from(user.version), *version);
            }
            _ => anyhow::bail!("unexpected event: {:?}", events[0]),
        }
        Ok(())
    }

    #[test]
    fn test_from_events() -> anyhow::Result<()> {
        let user_id = UserId::new_for_testing();
        let user_name1 = UserName::new_for_testing();
        let user_name2 = UserName::new_for_testing();
        let user = User::from_events(vec![
            UserEvent::Created(UserCreated {
                at: "2020-01-02T15:16:17.000Z".to_owned(),
                id: String::from(EventId::new()),
                name: String::from(user_name1.clone()),
                user_id: String::from(user_id.clone()),
                version: 1,
            }),
            UserEvent::Updated(UserUpdated {
                at: "2020-01-02T15:16:18.000Z".to_owned(),
                id: String::from(EventId::new()),
                name: String::from(user_name2.clone()),
                user_id: String::from(user_id.clone()),
                version: 2,
            }),
        ])?;
        assert_eq!(user.id, user_id);
        assert_eq!(user.name, user_name2);
        assert_eq!(user.version, Version::from(2));
        Ok(())
    }

    #[test]
    fn test_id() -> anyhow::Result<()> {
        let user_name = UserName::new_for_testing();
        let (user, _) = User::create(user_name)?;
        assert_eq!(user.id(), &user.id);
        Ok(())
    }

    #[test]
    fn test_version() -> anyhow::Result<()> {
        let user_name = UserName::new_for_testing();
        let (user, _) = User::create(user_name)?;
        assert_eq!(user.version(), user.version);
        Ok(())
    }
}
