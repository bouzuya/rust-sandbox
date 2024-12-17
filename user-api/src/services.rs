use std::{collections::BTreeMap, sync::Arc};

use tokio::sync::Mutex;

use crate::models::{user::User, user_id::UserId};

#[derive(Clone, Default)]
pub struct AppState {
    users: Arc<Mutex<BTreeMap<UserId, User>>>,
}

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
