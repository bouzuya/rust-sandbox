use std::{env, io};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

async fn post<T, U>(url: &str, body: &T) -> Result<U, reqwest::Error>
where
    T: Serialize + ?Sized,
    U: DeserializeOwned,
{
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json; charset=UTF8")
        .header("X-Accept", "application/json")
        .json(&body)
        .send()
        .await?;
    // TODO: check status code
    // <https://getpocket.com/developer/docs/authentication>
    println!("{:#?}", response);
    let response_body = response.json::<U>().await?;
    Ok(response_body)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Step 1: Obtain a platform consumer key
    let consumer_key = env::var("CONSUMER_KEY")?;
    let redirect_uri = "pocketapp1234:authorizationFinished";
    let state = "state1";

    // Step 2: Obtain a request token
    #[derive(Debug, Serialize)]
    struct OAuthRequestRequestBody<'a> {
        consumer_key: &'a str,
        redirect_uri: &'a str,
        state: Option<&'a str>,
    }
    #[derive(Debug, Deserialize)]
    struct OAuthRequestResponseBody {
        code: String,
    }
    let response_body: OAuthRequestResponseBody = post(
        "https://getpocket.com/v3/oauth/request",
        &OAuthRequestRequestBody {
            consumer_key: consumer_key.as_str(),
            redirect_uri,
            state: Some(state),
        },
    )
    .await?;
    println!("{:#?}", response_body);
    let request_token = response_body.code;
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
    #[derive(Debug, Serialize)]
    struct OAuthAuthorizeRequestBody<'a> {
        consumer_key: &'a str,
        code: &'a str,
    }
    #[derive(Debug, Deserialize)]
    struct OAuthAuthorizeResponseBody {
        access_token: String,
        username: String,
        state: Option<String>,
    }
    let response_body: OAuthAuthorizeResponseBody = post(
        "https://getpocket.com/v3/oauth/authorize",
        &OAuthAuthorizeRequestBody {
            consumer_key: consumer_key.as_str(),
            code: request_token.as_str(),
        },
    )
    .await?;
    println!("{:#?}", response_body);
    if response_body.state.as_deref() != Some(state) {
        // TODO: Error
        println!(
            "state does not match: expected {}, actual {:?}",
            state,
            response_body.state.as_deref()
        );
    }
    let access_token = response_body.access_token;
    println!("access_token = {}", access_token);
    let username = response_body.username;
    println!("username = {}", username);

    Ok(())
}
