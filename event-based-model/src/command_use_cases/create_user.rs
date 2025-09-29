pub struct CreateUserDeps {
    pub user_repository: std::sync::Arc<dyn crate::repositories::UserRepository + Send + Sync>,
}

#[derive(Debug)]
pub struct CreateUserInput {
    pub name: String,
}

#[derive(Debug)]
pub struct CreateUserOutput {
    pub id: String,
    pub version: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateUserError {
    #[error("create user")]
    CreateUser(#[source] crate::aggregates::UserError),
    #[error("invalid user name")]
    InvalidUserName(#[source] crate::value_objects::UserNameError),
    #[error("store")]
    Store(#[source] crate::repositories::UserRepositoryError),
}

pub async fn create_user(
    CreateUserDeps { user_repository }: CreateUserDeps,
    CreateUserInput { name }: CreateUserInput,
) -> Result<CreateUserOutput, CreateUserError> {
    let name =
        crate::value_objects::UserName::try_from(name).map_err(CreateUserError::InvalidUserName)?;

    let (created, user_events) =
        crate::aggregates::User::create(name).map_err(CreateUserError::CreateUser)?;

    user_repository
        .store(None, user_events)
        .await
        .map_err(CreateUserError::Store)?;

    Ok(CreateUserOutput {
        id: String::from(created.id()),
        version: u32::from(created.version()),
    })
}
