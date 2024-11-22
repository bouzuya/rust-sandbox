use crate::user::User;
use crate::AppState;
use axum::{extract::State, routing::post, Json};

use super::Error;

#[derive(serde::Serialize)]
struct CreateUserResponse {
    user_id: String,
    user_secret: String,
}

async fn create_user(State(app_state): State<AppState>) -> Result<Json<CreateUserResponse>, Error> {
    let mut users = app_state.users.lock()?;
    let (user, raw) = User::new().map_err(|_| Error::Server)?;
    users.insert(user.id, user.clone());
    Ok(Json(CreateUserResponse {
        user_id: user.id.to_string(),
        user_secret: raw,
    }))
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/users", post(create_user))
}
