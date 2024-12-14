use crate::AppState;
use axum::{extract::State, routing::post, Json};

use super::Error;

#[derive(serde::Serialize)]
struct ResponseBody {
    session_token: String,
}

impl std::fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseBody")
            .field("session_token", &"[FILTERED]")
            .finish()
    }
}

#[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(app_state))]
async fn handle(State(app_state): State<AppState>) -> Result<Json<ResponseBody>, Error> {
    let session_token = app_state
        .create_anonymous_session()
        .await
        .map_err(Error::Server)?;
    Ok(Json(ResponseBody { session_token }))
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/sessions", post(handle))
}
