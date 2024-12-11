use crate::session_id_extractor::SessionIdExtractor;
use crate::AppState;
use axum::{extract::State, routing::get, Json};

use super::Error;

#[derive(Debug, serde::Serialize)]
struct ResponseBody {
    user_id: String,
}

#[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(app_state))]
async fn handle(
    SessionIdExtractor(session_id): SessionIdExtractor,
    State(app_state): State<AppState>,
) -> Result<Json<ResponseBody>, Error> {
    tracing::debug!("get user");
    let mut sessions = app_state.sessions.lock().await;
    let session = sessions
        .get_mut(&session_id)
        .ok_or_else(|| Error::Client(anyhow::anyhow!("sign_up session not found")))?;
    Ok(Json(ResponseBody {
        user_id: session.user_id.to_string(),
    }))
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/users/me", get(handle))
}
