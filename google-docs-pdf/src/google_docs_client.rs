pub mod v1;

use self::v1::documents::request::Request;

use crate::token_source::TokenSource;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateRequestBody {
    pub requests: Option<Vec<Request>>,
    // TODO:
}

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

    // <https://developers.google.com/docs/api/reference/rest/v1/documents/batchUpdate>
    pub async fn v1_documents_batch_update<S: AsRef<str>>(
        &self,
        document_id: S,
        body: &BatchUpdateRequestBody,
    ) -> anyhow::Result<String> {
        let token = self.token_source.token().await?;
        let request = self
            .client
            .request(
                reqwest::Method::POST,
                format!(
                    "{}/v1/documents/{}:batchUpdate",
                    self.service_endpoint,
                    document_id.as_ref()
                ),
            )
            .header("Authorization", format!("Bearer {}", token))
            .json(body)
            .build()?;
        let response = self.client.execute(request).await?;
        if !response.status().is_success() {
            anyhow::bail!("{:?}", response.status());
        }
        Ok(response.text().await?)
    }
    // <https://developers.google.com/docs/api/reference/rest/v1/documents/get>
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

    pub(crate) fn test_serde<
        T: std::fmt::Debug + PartialEq + serde::Serialize + serde::de::DeserializeOwned,
    >(
        s: &str,
        v: T,
    ) -> anyhow::Result<()> {
        assert_eq!(serde_json::from_str::<'_, T>(s)?, v);
        assert_eq!(
            serde_json::from_str::<'_, serde_json::Value>(&serde_json::to_string(&v)?)?,
            serde_json::from_str::<'_, serde_json::Value>(s)?
        );
        Ok(())
    }

    #[test]
    fn test() {
        fn assert_impls<T: Clone + Send + Sync>() {}
        assert_impls::<GoogleDocsClient>();
    }
}
