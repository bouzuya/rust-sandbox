use anyhow::Context as _;

// <https://developers.google.com/identity/openid-connect/openid-connect?hl=ja#discovery>
#[derive(serde::Deserialize)]
pub struct DiscoveryDocument {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
}

impl DiscoveryDocument {
    pub async fn fetch() -> anyhow::Result<Self> {
        fetch_discovery_document().await
    }
}

async fn fetch_discovery_document() -> anyhow::Result<DiscoveryDocument> {
    let response = reqwest::Client::new()
        .get("https://accounts.google.com/.well-known/openid-configuration")
        .send()
        .await
        .context("get discovery_document")?;
    if !response.status().is_success() {
        anyhow::bail!(
            "discovery_document response status is not success. status={}",
            response.status()
        );
    }
    let response_body = response
        .text()
        .await
        .context("read discovery_document response body")?;
    // println!("{}", response_body);
    let discovery_document = serde_json::from_str::<DiscoveryDocument>(&response_body)
        .context("deserialize discovery_document")?;
    Ok(discovery_document)
}
