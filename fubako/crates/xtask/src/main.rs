#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Create a new page
    New,
    /// Start a local preview server
    Preview,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::New => new().await,
        Subcommand::Preview => preview().await,
    }
}

async fn new() -> anyhow::Result<()> {
    let dir = std::path::PathBuf::from("data");
    let now = chrono::Utc::now();
    let id = now.format("%Y%m%dT%H%M%SZ").to_string();
    let path = dir.join(id).with_extension("md");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(&path, "")?;
    println!("Created new page: {}", path.display());
    Ok(())
}

async fn preview() -> anyhow::Result<()> {
    struct Id(String);

    impl std::fmt::Display for Id {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<'de> serde::de::Deserialize<'de> for Id {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'vi> serde::de::Visitor<'vi> for Visitor {
                type Value = Id;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a string matching the ID format")
                }

                fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    (v.len() == "00000000T000000Z".len()
                        && v.chars().all(|c| matches!(c, '0'..='9' | 'T' | 'Z')))
                    .then_some(v)
                    .map(Id)
                    .ok_or_else(|| E::custom("invalid ID format"))
                }
            }

            deserializer.deserialize_string(Visitor)
        }
    }

    async fn get(
        axum::extract::Path(id): axum::extract::Path<Id>,
    ) -> Result<axum::response::Html<String>, axum::http::StatusCode> {
        let id_str = id.to_string();
        let md = std::fs::read_to_string(format!("data/{}.md", &id_str))
            .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
        let parser = pulldown_cmark::Parser::new(&md);
        let mut html = String::new();
        html.push_str("<!DOCTYPE html><html><body>");
        html.push_str(r#"<nav><ol><li><a href="/">/</a></li><li><a href=""#);
        html.push_str(id_str.as_str());
        html.push_str(r#"">"#);
        html.push_str(id_str.as_str());
        html.push_str("</a></li></ol></nav>");
        pulldown_cmark::html::push_html(&mut html, parser);
        html.push_str("</body></html>");
        Ok(axum::response::Html(html))
    }

    async fn list() -> Result<axum::response::Html<String>, axum::http::StatusCode> {
        let read_dir =
            std::fs::read_dir("data").map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut ids = Vec::new();
        for dir_entry in read_dir {
            let dir_entry = dir_entry.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
            let path_buf = dir_entry.path();
            let file_stem = path_buf
                .file_stem()
                .ok_or_else(|| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
            let id = file_stem
                .to_str()
                .ok_or_else(|| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
            ids.push(id.to_string());
        }
        ids.sort();
        let mut html = String::new();
        html.push_str("<!DOCTYPE html><html><body>");
        html.push_str(r#"<nav><ol><li><a href="/">/</a></li></ol></nav>"#);
        html.push_str("<h1>Index</h1>");
        if !ids.is_empty() {
            html.push_str("<ul>");
            for id in ids {
                html.push_str("<li>");
                html.push_str(r#"<a href="/"#);
                html.push_str(id.as_str());
                html.push_str(r#"">"#);
                html.push_str(id.as_str());
                html.push_str("</a>");
                html.push_str("</li>")
            }
            html.push_str("</ul>");
        }
        html.push_str("</body></html>");
        Ok(axum::response::Html(html))
    }

    let router = axum::Router::new()
        .route("/", axum::routing::get(list))
        .route("/{id}", axum::routing::get(get));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
