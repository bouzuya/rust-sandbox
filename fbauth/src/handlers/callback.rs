use crate::AppState;
use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
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
    State(app_state): State<AppState>,
    Query(query): Query<CallbackQueryParams>,
) -> Result<Html<String>, Error> {
    // FIXME: check state
    println!("query.state = {}", query.state);
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
    axum::Router::new().route("/callback", get(callback))
}
