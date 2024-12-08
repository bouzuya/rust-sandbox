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

async fn handle(State(app_state): State<AppState>) -> Result<Json<ResponseBody>, Error> {
    let mut users = app_state.users.lock().await;
    let (user, raw) = User::new()
        .context("create_user User::new")
        .map_err(Error::Server)?;
    users.insert(user.id, user.clone());
    Ok(Json(ResponseBody {
        user_id: user.id.to_string(),
        user_secret: raw,
    }))
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/users", post(handle))
}
