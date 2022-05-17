use std::{env, io};

use hyper::StatusCode;
use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("request {0}")]
    Request(#[from] reqwest::Error),
    #[error(
        "status X-Error: {x_error:?}, X-Error-Code: {x_error_code:?}, HTTP Status: {status_code}"
    )]
    Status {
        status_code: u16,
        x_error_code: Option<String>,
        x_error: Option<String>,
    },
}

fn check_status_code(response: &Response) -> Option<Error> {
    let status = response.status();
    if status == StatusCode::OK {
        return None;
    }

    let headers = response.headers();
    let x_error_code = headers.get("X-Error-Code");
    let x_error = headers.get("X-Error");
    Some(Error::Status {
        status_code: status.as_u16(),
        x_error_code: x_error_code
            .map(|v| v.to_str())
            .transpose()
            .unwrap()
            .map(|v| v.to_owned()),
        x_error: x_error
            .map(|v| v.to_str())
            .transpose()
            .unwrap()
            .map(|v| v.to_owned()),
    })
}

async fn post<T, U>(url: &str, body: &T) -> Result<U, Error>
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
    if let Some(error) = check_status_code(&response) {
        return Err(error);
    }
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
