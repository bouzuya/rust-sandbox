#[async_trait::async_trait]
pub trait TokenSource {
    async fn token(&self) -> anyhow::Result<String>;
}

#[derive(Clone)]
pub struct GoogleCloudAuthTokenSource {
    credential: google_cloud_auth::Credential,
}

impl GoogleCloudAuthTokenSource {
    pub async fn new<I>(scopes: I) -> anyhow::Result<Self>
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        Ok(Self {
            credential: google_cloud_auth::Credential::find_default(
                google_cloud_auth::CredentialConfig::builder()
                    .scopes(
                        scopes
                            .into_iter()
                            .map(|s| s.into())
                            .collect::<Vec<String>>(),
                    )
                    .build()?,
            )
            .await?,
        })
    }
}

#[async_trait::async_trait]
impl TokenSource for GoogleCloudAuthTokenSource {
    async fn token(&self) -> anyhow::Result<String> {
        Ok(self.credential.access_token().await?.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn assert_impls<T: Send + Sync>() {}
        assert_impls::<GoogleCloudAuthTokenSource>();
    }
}
