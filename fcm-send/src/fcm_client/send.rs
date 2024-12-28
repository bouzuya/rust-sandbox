use crate::fcm_client::{Error, Message};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PathParameters {
    pub parent: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RequestBody {
    // validate_only: Option<bool>,
    pub message: Message,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ResponseBody(pub Message);

/// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages/send>
pub(crate) async fn handle(
    client: &crate::FcmClient,
    PathParameters { parent }: PathParameters,
    request_body: RequestBody,
) -> Result<ResponseBody, Error> {
    let token = client
        .token_source
        .token()
        .await
        .map_err(Error::Authorization)?;
    let response = client
        .client
        .post(format!(
            "https://fcm.googleapis.com/v1/{}/messages:send",
            parent
        ))
        .header("Authorization", token)
        .json(&request_body)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(Into::into)
        .map_err(Error::Request)?;
    if response.status().is_success() {
        let response_body = response
            .text()
            .await
            .map_err(Into::into)
            .map_err(Error::ReadResponse)?;
        let response_body = serde_json::from_str(&response_body)
            .map_err(Into::into)
            .map_err(Error::Deserialize)?;
        Ok(response_body)
    } else {
        let response_body = response
            .text()
            .await
            .map_err(Into::into)
            .map_err(Error::ReadResponse)?;
        Err(Error::ErrorResponse(response_body))
    }
}
