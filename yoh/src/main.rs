#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let response = reqwest::get("https://bouzuya.net/").await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}
