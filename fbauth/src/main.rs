use axum::{response::Html, routing::get, Router};
// use tower_http::services::ServeDir;

async fn root() -> Html<String> {
    let client_id = "FIXME";
    let nonce = "FIXME";
    let redirect_uri = "FIXME";
    let state = "FIXME";

    let mut url = url::Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let router = Router::new().nest_service("/", ServeDir::new("public"));
    let router = Router::new().route("/", get(root));
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
