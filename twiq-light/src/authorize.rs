use std::{collections::HashMap, env, io};

use anyhow::{bail, Context};
use rand::{rngs::ThreadRng, RngCore};
use reqwest::{Client, Method};
use sha2::{Digest, Sha256};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use tracing::debug;
use url::Url;

use crate::store::{Token, TweetQueueStore};

// <https://www.rfc-editor.org/rfc/rfc6749#section-5.1>
#[derive(Debug, serde::Deserialize)]
struct AccessTokenResponse {
    access_token: String, // ...
    #[allow(dead_code)]
    token_type: String, // "bearer"
    expires_in: Option<u32>, // 7200
    #[allow(dead_code)]
    scope: Option<String>, // "tweet.write users.read tweet.read offline.access"
    refresh_token: Option<String>, // ...
}

impl AccessTokenResponse {
    pub fn try_into(self, unix_timestamp: i64) -> anyhow::Result<Token> {
        let now = OffsetDateTime::from_unix_timestamp(unix_timestamp)?;

        let access_token = self.access_token;
        let expires_in = self.expires_in.context("expires_in is none")?;
        let refresh_token = self.refresh_token.context("refresh_token is none")?;

        let expires = now + Duration::seconds(i64::from(expires_in));
        let expires = expires.format(&Rfc3339)?;
        Ok(Token {
            access_token,
            expires,
            refresh_token,
        })
    }
}

async fn token_request(
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
    code_verifier: &str,
) -> anyhow::Result<AccessTokenResponse> {
    let response = Client::builder()
        .build()?
        .request(Method::POST, "https://api.twitter.com/2/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(client_id, Some(client_secret))
        .form(&{
            let mut form = HashMap::new();
            form.insert("code", code);
            form.insert("grant_type", "authorization_code");
            form.insert("redirect_uri", redirect_uri);
            form.insert("code_verifier", code_verifier);
            form
        })
        .send()
        .await?;
    Ok(response.json().await?)
}

pub async fn run(store: TweetQueueStore) -> anyhow::Result<()> {
    let response_type = "code";
    let client_id = env::var("TWITTER_CLIENT_ID")?;
    let client_secret = env::var("TWITTER_CLIENT_SECRET")?;
    let redirect_uri = env::var("TWITTER_REDIRECT_URI")?;

    let mut rng = ThreadRng::default();
    let mut state_buf = vec![0; 96];
    rng.fill_bytes(&mut state_buf);
    let mut code_verifier_buf = vec![0; 96];
    rng.fill_bytes(&mut code_verifier_buf);
    let base64_engine = base64::engine::fast_portable::FastPortable::from(
        &base64::alphabet::URL_SAFE,
        base64::engine::fast_portable::NO_PAD,
    );

    let scope = "tweet.read%20tweet.write%20users.read%20offline.access";
    let state = base64::encode_engine(&state_buf, &base64_engine);
    let code_verifier = base64::encode_engine(&code_verifier_buf, &base64_engine);
    let code_challenge =
        base64::encode_engine(Sha256::digest(code_verifier.as_bytes()), &base64_engine);
    let code_challenge_method = "s256";
    let url = format!(
            "https://twitter.com/i/oauth2/authorize?response_type={}&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method={}",
            response_type,
            client_id,
            redirect_uri,
            scope,
            state,
            code_challenge,
            code_challenge_method,
        );

    println!("{}", url);

    // read redirect_uri
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    // parse url and check state
    let url = Url::parse(buffer.trim())?;
    let (_, response_state) = url.query_pairs().find(|(k, _)| k == "state").unwrap();
    if response_state.to_string().as_str() != state {
        bail!("state does not match");
    }
    let (_, code) = url.query_pairs().find(|(k, _)| k == "code").unwrap();
    let code = code.to_string();

    let access_token_response = token_request(
        &client_id,
        &client_secret,
        code.as_str(),
        &redirect_uri,
        &code_verifier,
    )
    .await?;

    debug!("{:?}", access_token_response);

    let token = access_token_response.try_into(OffsetDateTime::now_utc().unix_timestamp())?;

    store.write_token(&token).await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn base64_test() {
        let mut rng = ThreadRng::default();
        let mut state_buf = vec![0; 96];
        rng.fill_bytes(&mut state_buf);
        let base64_engine = base64::engine::fast_portable::FastPortable::from(
            &base64::alphabet::URL_SAFE,
            base64::engine::fast_portable::NO_PAD,
        );
        let state = base64::encode_engine(&state_buf, &base64_engine);
        assert_eq!(state.len(), 128);
    }
}
