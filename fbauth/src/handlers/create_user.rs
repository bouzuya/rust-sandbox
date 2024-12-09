use crate::user::User;
use crate::AppState;
use anyhow::Context as _;
use axum::{extract::State, routing::post, Json};

use super::Error;

#[derive(serde::Serialize)]
struct ResponseBody {
    user_id: String,
    user_secret: String,
}

impl std::fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseBody")
            .field("user_id", &self.user_id)
            .field("user_secret", &"[FILTERED]")
            .finish()
    }
}

#[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(app_state))]
async fn handle(State(app_state): State<AppState>) -> Result<Json<ResponseBody>, Error> {
    tracing::debug!("create user");
    let mut users = app_state.users.lock().await;
    let (user, raw) = User::new()
        .context("create_user User::new")
        .map_err(Error::Server)?;
    users.insert(user.id, user.clone());
    tracing::debug!(user_id = user.id.to_string(), "user created");
    Ok(Json(ResponseBody {
        user_id: user.id.to_string(),
        user_secret: raw,
    }))
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/users", post(handle))
}
