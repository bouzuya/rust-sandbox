mod aggregates;
mod command_use_cases;
mod event;
mod in_memory_impls;
mod query_models;
mod query_use_cases;
mod readers;
mod repositories;
mod value_objects;

#[tokio::main]
async fn main() {
    sample1().unwrap();
    sample2().await.unwrap();
}

#[derive(Debug, thiserror::Error)]
enum Sample1Error {
    #[error("user name")]
    UserName(#[from] self::value_objects::UserNameError),
    #[error("user")]
    User(#[from] self::aggregates::UserError),
}

fn sample1() -> Result<(), Sample1Error> {
    let name1 = self::value_objects::UserName::try_from("Alice".to_owned())?;
    let (created, create_events) = self::aggregates::User::create(name1)?;
    let name2 = self::value_objects::UserName::try_from("Bob".to_owned())?;
    let (updated, update_events) = created.update(name2)?;
    let replayed =
        self::aggregates::User::from_events(create_events.into_iter().chain(update_events))?;
    assert_eq!(updated, replayed);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Sample2Error {
    #[error("create user")]
    CreateUser(#[from] self::command_use_cases::CreateUserError),
    #[error("list users")]
    ListUsers(#[from] self::query_use_cases::ListUsersError),
    #[error("user id")]
    UserId(#[from] self::value_objects::UserIdError),
    #[error("user name")]
    UserName(#[from] self::value_objects::UserNameError),
    #[error("user")]
    User(#[from] self::aggregates::UserError),
    #[error("user not found")]
    UserNotFound,
    #[error("user repository")]
    UserRepository(#[from] self::repositories::UserRepositoryError),
}

async fn sample2() -> Result<(), Sample2Error> {
    use crate::repositories::UserRepository;

    let event_store = in_memory_impls::InMemoryEventStore::new();
    let user_repository = std::sync::Arc::new(in_memory_impls::InMemoryUserRepository::new(
        event_store.clone(),
    ));
    let self::command_use_cases::CreateUserOutput { id, version } =
        self::command_use_cases::create_user(
            self::command_use_cases::CreateUserDeps {
                user_repository: user_repository.clone(),
            },
            self::command_use_cases::CreateUserInput {
                name: "Alice".to_owned(),
            },
        )
        .await?;

    assert!(!id.is_empty());
    assert_eq!(version, 1_u32);

    let found = user_repository
        .find(&crate::value_objects::UserId::try_from(id.clone())?)
        .await?
        .ok_or_else(|| Sample2Error::UserNotFound)?;
    assert_eq!(String::from(found.id()), id);
    assert_eq!(u32::from(found.version()), version);

    // TODO: read events and write query model

    let user_reader = std::sync::Arc::new(in_memory_impls::InMemoryUserReader::new());
    let self::query_use_cases::ListUsersOutput { items } = self::query_use_cases::list_users(
        self::query_use_cases::ListUsersDeps { user_reader },
        self::query_use_cases::ListUsersInput,
    )
    .await?;
    assert!(items.is_empty());

    Ok(())
}
