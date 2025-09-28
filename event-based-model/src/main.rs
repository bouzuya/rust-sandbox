use crate::repositories::UserRepository;

mod aggregates;
mod event;
mod repositories;
mod use_cases;
mod value_objects;

#[tokio::main]
async fn main() {
    sample1().unwrap();
    sample2().await.unwrap();
}

fn sample1() -> Result<(), Box<dyn std::error::Error>> {
    let name1 = self::value_objects::UserName::try_from("Alice".to_owned())?;
    let (created, create_events) = self::aggregates::User::create(name1)?;
    let name2 = self::value_objects::UserName::try_from("Bob".to_owned())?;
    let (updated, update_events) = created.update(name2)?;
    let replayed =
        self::aggregates::User::from_events(create_events.into_iter().chain(update_events))?;
    assert_eq!(updated, replayed);
    Ok(())
}

async fn sample2() -> Result<(), Box<dyn std::error::Error>> {
    let user_repository = std::sync::Arc::new(InMemoryUserRepository::new());
    let self::use_cases::CreateUserOutput { id, version } = self::use_cases::create_user(
        self::use_cases::CreateUserDeps {
            user_repository: user_repository.clone(),
        },
        self::use_cases::CreateUserInput {
            name: "Alice".to_owned(),
        },
    )
    .await?;

    assert!(!id.is_empty());
    assert_eq!(version, 1_u32);

    let found = user_repository
        .find(&crate::value_objects::UserId::try_from(id.clone())?)
        .await?
        .ok_or_else(|| "not found".to_owned())?;
    assert_eq!(String::from(found.id()), id);
    assert_eq!(u32::from(found.version()), version);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum InMemoryUserRepositoryError {
    #[error("user already exists (id={0})")]
    UserAlreadyExists(String),
    #[error("user already updated (id={0})")]
    UserAlreadyUpdated(String),
    #[error("user not found (id={0})")]
    UserNotFound(String),
}

impl From<InMemoryUserRepositoryError> for self::repositories::UserRepositoryError {
    fn from(e: InMemoryUserRepositoryError) -> Self {
        self::repositories::UserRepositoryError(Box::new(e))
    }
}

struct InMemoryUserRepository {
    store: std::sync::Mutex<std::collections::HashMap<String, Vec<event::UserEvent>>>,
}

impl InMemoryUserRepository {
    fn new() -> Self {
        Self {
            store: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl self::repositories::UserRepository for InMemoryUserRepository {
    async fn find(
        &self,
        id: &crate::value_objects::UserId,
    ) -> Result<Option<crate::aggregates::User>, self::repositories::UserRepositoryError> {
        let store = self.store.lock().unwrap();

        let id = String::from(id);
        match store.get(&id) {
            None => Ok(None),
            Some(events) => {
                let user = crate::aggregates::User::from_events(events.into_iter().cloned())
                    .map_err(|e| self::repositories::UserRepositoryError(Box::new(e)))?;
                Ok(Some(user))
            }
        }
    }

    async fn store(
        &self,
        version: Option<crate::value_objects::Version>,
        user_events: Vec<crate::event::UserEvent>,
    ) -> Result<(), self::repositories::UserRepositoryError> {
        if user_events.is_empty() {
            return Ok(());
        }

        let user_id = event_to_user_id(&user_events[0]);
        let mut store = self.store.lock().unwrap();

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

fn event_to_user_id(e: &event::UserEvent) -> String {
    match e {
        event::UserEvent::Created(e) => e.user_id.clone(),
        event::UserEvent::Updated(e) => e.user_id.clone(),
    }
}

fn event_to_version(e: &event::UserEvent) -> u32 {
    match e {
        event::UserEvent::Created(e) => e.version,
        event::UserEvent::Updated(e) => e.version,
    }
}
