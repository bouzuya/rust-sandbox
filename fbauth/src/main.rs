mod app;
mod discovery_document;
mod handlers;
mod session;
mod session_id;
mod session_id_extractor;
mod user;
mod user_id;
mod user_secret;

use app::AppState;

use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let state = AppState::new().await?;
    let router = handlers::route()
        .with_state(state)
        .layer(TraceLayer::new_for_http());
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

    #[test]
    fn test_jwt() -> anyhow::Result<()> {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub(crate) struct Claims {
            /// session_id
            exp: u64,
            sid: String,
            sub: String,
            // ...
        }

        let exp = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs()
            + 60 * 60;

        let pem = include_bytes!("../private_key.pem");
        let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(pem)?;
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256),
            &Claims {
                exp,
                sid: "sid1".to_owned(),
                sub: "uid1".to_owned(),
            },
            &encoding_key,
        )?;
        let pem = include_bytes!("../public_key.pem");
        let decoding_key = jsonwebtoken::DecodingKey::from_rsa_pem(pem)?;
        let jsonwebtoken::TokenData { header: _, claims } = jsonwebtoken::decode::<Claims>(
            &token,
            &decoding_key,
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
        )?;

        assert_ne!(format!("{:?}", claims), "");

        Ok(())
    }
}
