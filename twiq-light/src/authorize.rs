use std::{collections::HashMap, env, io};

use anyhow::bail;
use rand::{rngs::ThreadRng, RngCore};
use reqwest::{Client, Method};
use sha2::{Digest, Sha256};
use tracing::debug;
use url::Url;

pub async fn run() -> anyhow::Result<()> {
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

    // <https://www.rfc-editor.org/rfc/rfc6749#section-5.1>
    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    struct AccessTokenResponse {
        access_token: String,          // ...
        token_type: String,            // "bearer"
        expires_in: Option<u32>,       // 7200
        scope: Option<String>,         // "tweet.write users.read tweet.read offline.access"
        refresh_token: Option<String>, // ...
    }

    let response = Client::builder()
        .build()?
        .request(Method::POST, "https://api.twitter.com/2/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(&client_id, Some(&client_secret))
        .form(&{
            let mut form = HashMap::new();
            form.insert("code", code.as_str());
            form.insert("grant_type", "authorization_code");
            form.insert("redirect_uri", &redirect_uri);
            form.insert("code_verifier", &code_verifier);
            form
        })
        .send()
        .await?;
    let access_token_response: AccessTokenResponse = response.json().await?;

    debug!("access_token_response={:?}", access_token_response);

    let refresh_token = &access_token_response.refresh_token.expect(
        "If offline.access is specified, a refresh_token must be included in the response.",
    );

    // refresh (access_)token
    let response = Client::builder()
        .build()?
        .request(Method::POST, "https://api.twitter.com/2/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(&client_id, Some(&client_secret))
        .form(&{
            let mut form = HashMap::new();
            form.insert("grant_type", "refresh_token");
            form.insert("refresh_token", refresh_token.as_str());
            form
        })
        .send()
        .await?;
    let access_token_response: AccessTokenResponse = response.json().await?;

    debug!("access_token_response={:?}", access_token_response);

    // FIXME: use access_token
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
