#[tokio::main]
async fn main() {
    sample2().await.unwrap();
}

#[derive(Debug, thiserror::Error)]
enum Sample2Error {
    #[error("create user")]
    CreateUser(#[from] event_based_model::command_use_cases::CreateUserError),
    #[error("list users")]
    ListUsers(#[from] event_based_model::query_use_cases::ListUsersError),
    #[error("user id")]
    UserId(#[from] event_based_model::value_objects::UserIdError),
    #[error("user name")]
    UserName(#[from] event_based_model::value_objects::UserNameError),
    #[error("user")]
    User(#[from] event_based_model::aggregates::UserError),
    #[error("user not found")]
    UserNotFound,
    #[error("user repository")]
    UserRepository(#[from] event_based_model::repositories::UserRepositoryError),
    #[error("user writer")]
    UserWriter(#[source] event_based_model::writers::UserWriterError),
}

async fn sample2() -> Result<(), Sample2Error> {
    use event_based_model::repositories::UserRepository;

    let event_store = event_based_model::in_memory_impls::InMemoryEventStore::new();
    let user_repository = std::sync::Arc::new(
        event_based_model::in_memory_impls::InMemoryUserRepository::new(event_store.clone()),
    );
    let event_based_model::command_use_cases::CreateUserOutput { id, version } =
        event_based_model::command_use_cases::create_user(
            event_based_model::command_use_cases::CreateUserDeps {
                user_repository: user_repository.clone(),
            },
            event_based_model::command_use_cases::CreateUserInput {
                name: "Alice".to_owned(),
            },
        )
        .await?;

    assert!(!id.is_empty());
    assert_eq!(version, 1_u32);

    let found = user_repository
        .find(&event_based_model::value_objects::UserId::try_from(
            id.clone(),
        )?)
        .await?
        .ok_or_else(|| Sample2Error::UserNotFound)?;
    assert_eq!(String::from(found.id()), id);
    assert_eq!(u32::from(found.version()), version);

    let read_model_store = event_based_model::in_memory_impls::InMemoryReadModelStore::new();
    let user_writer = event_based_model::in_memory_impls::InMemoryUserWriter::new(
        event_store,
        read_model_store.clone(),
    );
    event_based_model::writers::UserWriter::update(&user_writer, found.id())
        .await
        .map_err(Sample2Error::UserWriter)?;

    let user_reader = std::sync::Arc::new(
        event_based_model::in_memory_impls::InMemoryUserReader::new(read_model_store),
    );
    let event_based_model::query_use_cases::ListUsersOutput { items } =
        event_based_model::query_use_cases::list_users(
            event_based_model::query_use_cases::ListUsersDeps { user_reader },
            event_based_model::query_use_cases::ListUsersInput,
        )
        .await?;
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0],
        event_based_model::query_use_cases::ListUsersOutputItem {
            id,
            name: "Alice".to_owned(),
            version: 1,
        }
    );

    Ok(())
}
