mod access_token;
mod authorization;
mod retrieve;

pub use access_token::*;
pub use authorization::*;
use hyper::StatusCode;
use reqwest::Response;
pub use retrieve::*;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
