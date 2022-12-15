use std::{collections::HashMap, env, io};

use anyhow::bail;
use rand::{rngs::ThreadRng, RngCore};
use reqwest::{Client, Method};
use url::Url;

pub async fn run() -> anyhow::Result<()> {
    let response_type = "code";
    let client_id = env::var("TWITTER_CLIENT_ID")?;
    let client_secret = env::var("TWITTER_CLIENT_SECRET")?;
    let redirect_uri = env::var("TWITTER_REDIRECT_URI")?;

    let mut rng = ThreadRng::default();
    let mut state_buf = vec![0; 96];
    rng.fill_bytes(&mut state_buf);
    let base64_engine = base64::engine::fast_portable::FastPortable::from(
        &base64::alphabet::URL_SAFE,
        base64::engine::fast_portable::NO_PAD,
    );

    // FIXME: scope offline.access
    let scope = "tweet.read%20tweet.write%20users.read%20offline.access";
    let state = base64::encode_engine(&state_buf, &base64_engine);
    // FIXME: code_challenge_method s256
    let code_challenge = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
    let code_verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
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

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let url = Url::parse(buffer.trim())?;
    let (_, response_state) = url.query_pairs().find(|(k, _)| k == "state").unwrap();
    if response_state.to_string().as_str() != state {
        bail!("state does not match");
    }
    let (_, code) = url.query_pairs().find(|(k, _)| k == "code").unwrap();
    let code = code.to_string();

    let response = Client::builder()
        .build()?
        .request(Method::POST, "https://api.twitter.com/2/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(&client_id, Some(client_secret))
        .form(&{
            let mut form = HashMap::new();
            form.insert("code", code.as_str());
            form.insert("grant_type", "authorization_code");
            form.insert("client_id", &client_id);
            form.insert("redirect_uri", &redirect_uri);
            form.insert("code_verifier", code_verifier);
            form
        })
        .send()
        .await?;

    // FIXME: store access_token and refresh_token
    assert_eq!(format!("{:?}", response.text().await?), "");
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
