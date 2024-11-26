use crate::{session_id_extractor::SessionIdExtractor, AppState};
use axum::{
    extract::{Query, State},
    response::Html,
};

use super::Error;

#[derive(serde::Deserialize)]
struct CallbackQueryParams {
    // authuser: String,
    code: String,
    // hd: String,
    // prompt: String
    // scope: String,
    state: String,
}

#[derive(serde::Serialize)]
struct TokenRequestBody {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    grant_type: String,
}

async fn callback(
    SessionIdExtractor(session_id): SessionIdExtractor,
    Query(query): Query<CallbackQueryParams>,
    State(app_state): State<AppState>,
) -> Result<Html<String>, Error> {
    let sessions = app_state.sessions.lock().await;
    let session = sessions.get(&session_id).ok_or_else(|| Error::Client)?;
    if session.state != Some(query.state) {
        return Err(Error::Client);
    }

    let redirect_uri = "http://localhost:3000/callback".to_owned();

    let response = reqwest::Client::new()
        .post(app_state.token_endpoint)
        .json(&TokenRequestBody {
            code: query.code,
            client_id: app_state.client_id,
            client_secret: app_state.client_secret,
            redirect_uri,
            grant_type: "authorization_code".to_owned(),
        })
        .send()
        .await
        .map_err(|_| Error::Server)?;
    if !response.status().is_success() {
        println!("status code = {}", response.status());
        println!(
            "response body = {}",
            response.text().await.map_err(|_| Error::Server)?
        );
        Ok(Html("ERROR".to_owned()))
    } else {
        let response_body = response.text().await.map_err(|_| Error::Server)?;
        Ok(Html(response_body))
    }
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/callback", axum::routing::get(callback))
}
