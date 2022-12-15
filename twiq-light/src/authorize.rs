use std::{collections::HashMap, env, io};

use reqwest::{Client, Method};
use url::Url;

use crate::store::TweetQueueStore;

pub async fn run() -> anyhow::Result<()> {
    let response_type = "code";
    let client_id = env::var("TWITTER_CLIENT_ID")?;
    let client_secret = env::var("TWITTER_CLIENT_SECRET")?;
    let redirect_uri = env::var("TWITTER_REDIRECT_URI")?;

    // FIXME: scope offline.access
    let scope = "tweet.read%20tweet.write%20users.read";
    // FIXME: random state
    let state = "abc";
    // FIXME: code_challenge_method s256
    let code_challenge = "challenge";
    let code_verifier = "challenge";
    let code_challenge_method = "plain";
    // let code_challenge = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
    // let code_verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
    // let code_challenge_method = "s256";
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
    // FIXME: check state
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
            form.insert("code_challenge_method", code_challenge_method);
            form
        })
        .send()
        .await?;

    // FIXME: store access_token and refresh_token
    assert_eq!(format!("{:?}", response.text().await?), "");
    Ok(())
}
