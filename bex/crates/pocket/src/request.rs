use hyper::StatusCode;
use reqwest::Response;
use serde::{de::DeserializeOwned, Serialize};

use crate::Error;

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

pub async fn post<T, U>(url: &str, body: &T) -> Result<U, Error>
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
