use crate::token_source::TokenSource;

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct File {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    // ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    // ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
    // ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // ...
}

// <https://developers.google.com/drive/api/reference/rest/v3>
#[derive(Clone)]
pub struct GoogleDriveClient {
    client: reqwest::Client,
    service_endpoint: String,
    token_source: std::sync::Arc<dyn Send + Sync + TokenSource>,
}

impl GoogleDriveClient {
    const SERVICE_ENDPOINT: &'static str = "https://www.googleapis.com";

    pub fn new<T: Send + Sync + TokenSource + 'static>(token_source: T) -> Self {
        Self {
            client: reqwest::Client::new(),
            service_endpoint: Self::SERVICE_ENDPOINT.to_string(),
            token_source: std::sync::Arc::new(token_source),
        }
    }

    // <https://developers.google.com/drive/api/reference/rest/v3/files/copy>
    pub async fn v3_files_copy<S: AsRef<str>>(
        &self,
        file_id: S,
        body: &File,
    ) -> anyhow::Result<String> {
        let token = self.token_source.token().await?;
        let request = self
            .client
            .request(
                reqwest::Method::POST,
                format!(
                    "{}/drive/v3/files/{}/copy",
                    self.service_endpoint,
                    file_id.as_ref()
                ),
            )
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .build()?;
        let response = self.client.execute(request).await?;
        if !response.status().is_success() {
            anyhow::bail!("{:?}", response.status());
        }
        Ok(response.text().await?)
    }

    // <https://developers.google.com/drive/api/reference/rest/v3/files/export>
    pub async fn v3_files_export<S: AsRef<str>, T: AsRef<str>>(
        &self,
        file_id: S,
        mime_type: T,
    ) -> anyhow::Result<Vec<u8>> {
        let token = self.token_source.token().await?;
        let request = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/drive/v3/files/{}/export",
                    self.service_endpoint,
                    file_id.as_ref()
                ),
            )
            .query(&[("mimeType", mime_type.as_ref())])
            .header("Authorization", format!("Bearer {}", token))
            .build()?;
        let response = self.client.execute(request).await?;
        if !response.status().is_success() {
            anyhow::bail!("{:?}", response.status());
        }
        Ok(response.bytes().await?.to_vec())
    }

    // <https://developers.google.com/drive/api/reference/rest/v3/files/get>
    pub async fn v3_files_get<S: AsRef<str>>(&self, file_id: S) -> anyhow::Result<String> {
        let token = self.token_source.token().await?;
        let request = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/drive/v3/files/{}",
                    self.service_endpoint,
                    file_id.as_ref()
                ),
            )
            .header("Authorization", format!("Bearer {}", token))
            .build()?;
        let response = self.client.execute(request).await?;
        if !response.status().is_success() {
            anyhow::bail!("{:?}", response.status());
        }
        Ok(response.text().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn assert_impls<T: Clone + Send + Sync>() {}
        assert_impls::<GoogleDriveClient>();
    }
}
