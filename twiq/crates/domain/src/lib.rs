#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_works() -> anyhow::Result<()> {
        #[derive(Debug, serde::Deserialize)]
        struct Json {
            date: String,
        }
        let response = reqwest::get("https://blog.bouzuya.net/2022/08/07/index.json").await?;
        let json: Json = response.json().await?;
        assert_eq!(json.date, "2022-08-07");
        Ok(())
    }
}
