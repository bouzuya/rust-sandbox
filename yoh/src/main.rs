#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .gzip(true)
        .build()?;
    let response = client
        .get("https://order.yodobashi.com/yc/login/index.html")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
        )
        .header("Accept-Language", "ja")
        .send()
        .await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}
