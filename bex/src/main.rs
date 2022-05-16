use std::{collections::HashMap, env, future::Future, io};

use anyhow::Context;
use reqwest::Response;
use serde::Serialize;

fn post<T>(url: &str, body: &T) -> impl Future<Output = Result<Response, reqwest::Error>>
where
    T: Serialize + ?Sized,
{
    let client = reqwest::Client::new();
    client
        .post(url)
        .header("Content-Type", "application/json; charset=UTF8")
        .header("X-Accept", "application/json")
        .json(&body)
        .send()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Step 1: Obtain a platform consumer key
    let consumer_key = env::var("CONSUMER_KEY")?;
    let redirect_uri = "pocketapp1234:authorizationFinished";
    let state = "state1";

    // Step 2: Obtain a request token
    let url = "https://getpocket.com/v3/oauth/request";
    let mut body = HashMap::new();
    body.entry("consumer_key").or_insert(consumer_key.as_str());
    body.entry("redirect_uri").or_insert(redirect_uri);
    body.entry("state").or_insert(state);
    let resp = post(url, &body).await?;
    // TODO: check status code
    // <https://getpocket.com/developer/docs/authentication>
    println!("{:#?}", resp);
    // TODO: deserialize
    let json = resp.json::<HashMap<String, String>>().await?;
    println!("{:?}", json);
    let request_token = json
        .get("code")
        .map(|code| code.as_str())
        .context("$.code not found")?;
    println!("request_token = {}", request_token);

    // Step 3: Redirect user to Pocket to continue authorization
    // TODO: encode uri component
    let redirect_url = format!(
        "https://getpocket.com/auth/authorize?request_token={}&redirect_uri={}",
        request_token, redirect_uri
    );
    println!("{}", redirect_url);

    // Step 4: Receive the callback from Pocket
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    // Step 5: Convert a request token into a Pocket access token
    let url = "https://getpocket.com/v3/oauth/authorize";
    let mut body = HashMap::new();
    body.entry("consumer_key").or_insert(consumer_key.as_str());
    body.entry("code").or_insert(request_token);
    let resp = post(url, &body).await?;
    println!("{:#?}", resp);

    // "{\"access_token\":\"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxx\",\"username\":\"xxxxxxx\",\"state\":\"state1\"}"
    let json = resp.json::<HashMap<String, String>>().await?;
    println!("{:?}", json);

    Ok(())
}
