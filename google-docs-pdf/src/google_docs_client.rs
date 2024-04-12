use crate::token_source::TokenSource;

// <https://developers.google.com/docs/api/reference/rest>
// - batchUpdate
// - create
// - get
#[derive(Clone)]
pub struct GoogleDocsClient {
    client: reqwest::Client,
    service_endpoint: String,
    token_source: std::sync::Arc<dyn Send + Sync + TokenSource>,
}

impl GoogleDocsClient {
    const SERVICE_ENDPOINT: &'static str = "https://docs.googleapis.com";

    pub fn new<T: Send + Sync + TokenSource + 'static>(token_source: T) -> Self {
        Self {
            client: reqwest::Client::new(),
            service_endpoint: Self::SERVICE_ENDPOINT.to_string(),
            token_source: std::sync::Arc::new(token_source),
        }
    }

    // <https://developers.google.com/docs/api/reference/rest/v1/documents/get>
    // TODO: response type
    pub async fn v1_documents_get<S: AsRef<str>>(&self, document_id: S) -> anyhow::Result<String> {
        let token = self.token_source.token().await?;
        let request = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/v1/documents/{}",
                    self.service_endpoint,
                    document_id.as_ref()
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
        assert_impls::<GoogleDocsClient>();
    }
}
