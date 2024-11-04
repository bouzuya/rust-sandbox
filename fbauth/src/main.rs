mod discovery_document;

use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
};
use discovery_document::DiscoveryDocument;
use tower_http::services::ServeDir;

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
) -> Html<String> {
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
        .unwrap();
    if !response.status().is_success() {
        println!("status code = {}", response.status());
        println!("response body = {}", response.text().await.unwrap());
        Html("ERROR".to_owned())
    } else {
        let response_body = response.text().await.unwrap();
        // let body = serde_json::from_str(&response_body).unwrap();
        Html(response_body)
    }
}

async fn root(State(app_state): State<AppState>) -> Html<String> {
    let client_id = &app_state.client_id;
    let nonce = "FIXME";
    let redirect_uri = "http://localhost:3000/callback";
    let state = "FIXME";

    let mut url = url::Url::parse(&app_state.authorization_endpoint).unwrap();
    url.query_pairs_mut()
        .clear()
        .append_pair("response_type", "code")
        .append_pair("client_id", client_id)
        .append_pair("scope", "openid email")
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("state", state)
        .append_pair("nonce", nonce);
    let authorization_url = url.to_string();

    Html(format!(
        r#"<html>
  <head>
    <title>Title</title>
  </head>
  <body>
    <p><a href="{}">Login</a></p>
  </body
</html>"#,
        authorization_url
    ))
}

#[derive(Clone)]
struct AppState {
    authorization_endpoint: String,
    client_id: String,
    client_secret: String,
    token_endpoint: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let DiscoveryDocument {
        authorization_endpoint,
        token_endpoint,
    } = DiscoveryDocument::fetch().await?;
    let client_id = std::env::var("CLIENT_ID")?;
    let client_secret = std::env::var("CLIENT_SECRET")?;

    println!("authorization_endpoint={}", authorization_endpoint);
    println!("client_id={}", client_id);
    println!("client_secret={}", client_secret);
    println!("token_endpoint={}", token_endpoint);

    let router = Router::new()
        .route("/", get(root))
        .route("/callback", get(callback))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(AppState {
            authorization_endpoint,
            client_id,
            client_secret,
            token_endpoint,
        });
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() -> anyhow::Result<()> {
        let url = url::Url::parse("https://accounts.google.com/o/oauth2/v2/auth")?;
        assert_eq!(
            url.to_string(),
            "https://accounts.google.com/o/oauth2/v2/auth"
        );
        Ok(())
    }
}
