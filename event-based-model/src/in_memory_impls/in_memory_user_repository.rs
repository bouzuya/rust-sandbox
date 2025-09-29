#[derive(Debug, thiserror::Error)]
enum InMemoryUserRepositoryError {
    #[error("user already exists (id={0})")]
    UserAlreadyExists(String),
    #[error("user already updated (id={0})")]
    UserAlreadyUpdated(String),
    #[error("user not found (id={0})")]
    UserNotFound(String),
}

impl From<InMemoryUserRepositoryError> for crate::repositories::UserRepositoryError {
    fn from(e: InMemoryUserRepositoryError) -> Self {
        crate::repositories::UserRepositoryError(Box::new(e))
    }
}

pub struct InMemoryUserRepository {
    event_store: super::InMemoryEventStore,
}

impl InMemoryUserRepository {
    pub fn new(event_store: super::InMemoryEventStore) -> Self {
        Self { event_store }
    }
}

#[async_trait::async_trait]
impl crate::repositories::UserRepository for InMemoryUserRepository {
    async fn find(
        &self,
        id: &crate::value_objects::UserId,
    ) -> Result<Option<crate::aggregates::User>, crate::repositories::UserRepositoryError> {
        let store = self.event_store.0.lock().unwrap();

        let id = String::from(id);
        match store.get(&id) {
            None => Ok(None),
            Some(events) => {
                let user = crate::aggregates::User::from_events(events.into_iter().cloned())
                    .map_err(|e| crate::repositories::UserRepositoryError(Box::new(e)))?;
                Ok(Some(user))
            }
        }
    }

    async fn store(
        &self,
        version: Option<crate::value_objects::Version>,
        user_events: Vec<crate::event::UserEvent>,
    ) -> Result<(), crate::repositories::UserRepositoryError> {
        if user_events.is_empty() {
            return Ok(());
        }

        let user_id = event_to_user_id(&user_events[0]);
        let mut store = self.event_store.0.lock().unwrap();

        match version {
            None => {
                // create
                if store.contains_key(&user_id) {
                    return Err(InMemoryUserRepositoryError::UserAlreadyExists(user_id))?;
                }

                store.insert(user_id.clone(), user_events);
            }
            Some(version) => {
                // update
                let stored = store
                    .get_mut(&user_id)
                    .ok_or_else(|| InMemoryUserRepositoryError::UserNotFound(user_id.clone()))?;

                if stored.last().map(event_to_version) != Some(u32::from(version)) {
                    return Err(InMemoryUserRepositoryError::UserAlreadyUpdated(user_id))?;
                }

                stored.extend(user_events);
            }
        }

        Ok(())
    }
}

fn event_to_user_id(e: &crate::event::UserEvent) -> String {
    match e {
        crate::event::UserEvent::Created(e) => e.user_id.clone(),
        crate::event::UserEvent::Updated(e) => e.user_id.clone(),
    }
}

fn event_to_version(e: &crate::event::UserEvent) -> u32 {
    match e {
        crate::event::UserEvent::Created(e) => e.version,
        crate::event::UserEvent::Updated(e) => e.version,
    }
}
