#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    // email
    // <https://cloud.google.com/compute/docs/metadata/predefined-metadata-keys>
    let url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/email";
    let request = client
        .request(reqwest::Method::GET, url)
        .header("Metadata-Flavor", "Google")
        .build()?;
    let response = client.execute(request).await?;
    println!("status: {:?}", response.status());
    let response_body = response.text().await?;
    println!("response_body: {:?}", response_body);
    let email = response_body;

    // token
    // <https://google.aip.dev/auth/4115>
    let url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
    let request = client
        .request(reqwest::Method::GET, url)
        .header("Metadata-Flavor", "Google")
        .build()?;
    let response = client.execute(request).await?;
    println!("status: {:?}", response.status());
    let response_body = response.text().await?;
    println!("response_body: {:?}", response_body);
    #[derive(Debug, serde::Deserialize)]
    struct AccessToken {
        access_token: String,
        expires_in: usize,
        token_type: String,
    }
    let access_token = serde_json::from_str::<AccessToken>(&response_body)?;
    println!("{:?}", access_token);

    // project-id
    // <https://cloud.google.com/compute/docs/metadata/predefined-metadata-keys>
    let url = "http://metadata.google.internal/computeMetadata/v1/project/project-id";
    let request = client
        .request(reqwest::Method::GET, url)
        .header("Metadata-Flavor", "Google")
        .build()?;
    let response = client.execute(request).await?;
    println!("status: {:?}", response.status());
    let response_body = response.text().await?;
    println!("response_body: {:?}", response_body);

    Ok(())
}
