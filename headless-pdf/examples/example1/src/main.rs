#[derive(serde::Serialize)]
struct Body {
    template: String,
    data: Data,
}

#[derive(serde::Serialize)]
struct Data {
    kana: String,
    name: String,
    photo_url: String,
    title: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/pdfs")
        .json(&Body {
            template: include_str!("../index.html.tmpl").to_owned(),
            data: Data {
                kana: "ぼうずや".to_owned(),
                name: "bouzuya".to_owned(),
                photo_url: "https://bouzuya.net/images/bouzuya-icon-v3.png".to_owned(),
                title: "履歴書".to_owned(),
            },
        })
        .send()
        .await?;
    println!("{:?}", response.status());
    std::fs::write("output.pdf", response.bytes().await?.to_vec())?;
    Ok(())
}
