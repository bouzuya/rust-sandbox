use std::{collections::BTreeMap, str::FromStr as _, sync::Arc};

use tokio::sync::Mutex;

use crate::models::{user::User, user_id::UserId};

#[derive(Clone, Default)]
pub struct AppState {
    users: Arc<Mutex<BTreeMap<UserId, User>>>,
}

// auth

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid user_id")]
    InvalidUserId(#[source] anyhow::Error),
    #[error("secret not match")]
    SecretNotMatch(UserId),
    #[error("user not found")]
    UserNotFound(UserId),
}

pub struct AuthInput {
    pub user_id: String,
    pub user_secret: String,
}

impl std::fmt::Debug for AuthInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthInput")
            .field("user_id", &self.user_id)
            .field("user_secret", &"[FILTERED]")
            .finish()
    }
}

#[derive(Debug)]
pub struct AuthOutput {
    pub user_id: String,
    pub user_name: String,
}

#[axum::async_trait]
pub trait AuthService {
    async fn auth(&self, input: AuthInput) -> Result<AuthOutput, AuthError>;
}

#[axum::async_trait]
impl AuthService for AppState {
    #[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(self))]
    async fn auth(
        &self,
        AuthInput {
            user_id,
            user_secret,
        }: AuthInput,
    ) -> Result<AuthOutput, AuthError> {
        let user_id = UserId::from_str(&user_id).map_err(AuthError::InvalidUserId)?;
        let users = self.users.lock().await;
        let user = users
            .get(&user_id)
            .ok_or_else(|| AuthError::UserNotFound(user_id))?;
        user.secret
            .verify(&user_secret)
            .map_err(|_| AuthError::SecretNotMatch(user_id))?;
        Ok(AuthOutput {
            user_id: user.id.to_string(),
            user_name: user.name.clone(),
        })
    }
}

// create_user

#[derive(Debug, thiserror::Error)]
pub enum CreateUserError {
    #[error("new user")]
    NewUser(#[source] anyhow::Error),
}

#[derive(Debug)]
pub struct CreateUserInput {
    pub name: String,
}

pub struct CreateUserOutput {
    pub user_id: String,
    pub user_name: String,
    pub user_secret: String,
}

impl std::fmt::Debug for CreateUserOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateUserOutput")
            .field("user_id", &self.user_id)
            .field("user_name", &self.user_name)
            .field("user_secret", &"[FILTERED]")
            .finish()
    }
}

#[axum::async_trait]
pub trait CreateUserService {
    async fn create_user(
        &self,
        input: CreateUserInput,
    ) -> Result<CreateUserOutput, CreateUserError>;
}

#[axum::async_trait]
impl CreateUserService for AppState {
    #[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(self))]
    async fn create_user(
        &self,
        CreateUserInput { name }: CreateUserInput,
    ) -> Result<CreateUserOutput, CreateUserError> {
        let (user, secret) = User::new(name).map_err(CreateUserError::NewUser)?;
        let mut users = self.users.lock().await;
        users.insert(user.id, user.clone());
        Ok(CreateUserOutput {
            user_id: user.id.to_string(),
            user_name: user.name,
            user_secret: secret,
        })
    }
}

// delete_user

#[derive(Debug, thiserror::Error)]
pub enum DeleteUserError {
    #[error("invalid user_id")]
    InvalidUserId(#[source] anyhow::Error),
}

#[derive(Debug)]
pub struct DeleteUserInput {
    pub user_id: String,
}

#[derive(Debug)]
pub struct DeleteUserOutput;

#[axum::async_trait]
pub trait DeleteUserService {
    async fn delete_user(
        &self,
        input: DeleteUserInput,
    ) -> Result<DeleteUserOutput, DeleteUserError>;
}

#[axum::async_trait]
impl DeleteUserService for AppState {
    #[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(self))]
    async fn delete_user(
        &self,
        DeleteUserInput { user_id }: DeleteUserInput,
    ) -> Result<DeleteUserOutput, DeleteUserError> {
        let user_id = UserId::from_str(&user_id).map_err(DeleteUserError::InvalidUserId)?;
        let mut users = self.users.lock().await;
        users.remove(&user_id);
        Ok(DeleteUserOutput)
    }
}

// get_user

#[derive(Debug, thiserror::Error)]
pub enum GetUserError {
    #[error("invalid user_id")]
    InvalidUserId(#[source] anyhow::Error),
    #[error("user not found")]
    UserNotFound(UserId),
}

#[derive(Debug)]
pub struct GetUserInput {
    pub user_id: String,
}

#[derive(Debug)]
pub struct GetUserOutput {
    pub user_id: String,
    pub user_name: String,
}

#[axum::async_trait]
pub trait GetUserService {
    async fn get_user(&self, input: GetUserInput) -> Result<GetUserOutput, GetUserError>;
}

#[axum::async_trait]
impl GetUserService for AppState {
    #[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(self))]
    async fn get_user(
        &self,
        GetUserInput { user_id }: GetUserInput,
    ) -> Result<GetUserOutput, GetUserError> {
        let user_id = UserId::from_str(&user_id).map_err(GetUserError::InvalidUserId)?;
        let users = self.users.lock().await;
        let user = users
            .get(&user_id)
            .ok_or_else(|| GetUserError::UserNotFound(user_id))?;
        Ok(GetUserOutput {
            user_id: user.id.to_string(),
            user_name: user.name.clone(),
        })
    }
}

// get_users

#[derive(Debug, thiserror::Error)]
#[error("get users")]
pub struct GetUsersError;

#[derive(Debug)]
pub struct GetUsersInput;

#[derive(Debug)]
pub struct GetUsersOutput {
    pub users: Vec<GetUsersOutputItem>,
}

#[derive(Debug)]
pub struct GetUsersOutputItem {
    pub user_id: String,
    pub user_name: String,
}

#[axum::async_trait]
pub trait GetUsersService {
    async fn get_users(&self, input: GetUsersInput) -> Result<GetUsersOutput, GetUsersError>;
}

#[axum::async_trait]
impl GetUsersService for AppState {
    #[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(self))]
    async fn get_users(&self, _: GetUsersInput) -> Result<GetUsersOutput, GetUsersError> {
        let users = self.users.lock().await;
        let users = users
            .values()
            .map(|it| GetUsersOutputItem {
                user_id: it.id.to_string(),
                user_name: it.name.clone(),
            })
            .collect::<Vec<GetUsersOutputItem>>();
        Ok(GetUsersOutput { users })
    }
}

// update_user

#[derive(Debug, thiserror::Error)]
pub enum UpdateUserError {
    #[error("invalid user_id")]
    InvalidUserId(#[source] anyhow::Error),
    #[error("user not found")]
    UserNotFound(UserId),
    #[error("user update")]
    UserUpdate(#[source] anyhow::Error),
}

#[derive(Debug)]
pub struct UpdateUserInput {
    pub name: String,
    pub user_id: String,
}

#[derive(Debug)]
pub struct UpdateUserOutput {
    pub user_id: String,
    pub user_name: String,
}

#[axum::async_trait]
pub trait UpdateUserService {
    async fn update_user(
        &self,
        input: UpdateUserInput,
    ) -> Result<UpdateUserOutput, UpdateUserError>;
}

#[axum::async_trait]
impl UpdateUserService for AppState {
    #[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(self))]
    async fn update_user(
        &self,
        UpdateUserInput { name, user_id }: UpdateUserInput,
    ) -> Result<UpdateUserOutput, UpdateUserError> {
        let user_id = UserId::from_str(&user_id).map_err(UpdateUserError::InvalidUserId)?;
        let mut users = self.users.lock().await;
        let user = users
            .get_mut(&user_id)
            .ok_or_else(|| UpdateUserError::UserNotFound(user_id))?;
        user.update(name).map_err(UpdateUserError::UserUpdate)?;
        Ok(UpdateUserOutput {
            user_id: user.id.to_string(),
            user_name: user.name.clone(),
        })
    }
}
