mod page_meta;

use anyhow::Context;

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
    let config = load_config().await?;
    let dir = config.data_dir;
    let now = chrono::Utc::now();
    let id = now.format("%Y%m%dT%H%M%SZ").to_string();
    let path = dir.join(id).with_extension("md");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(&path, "")?;
    println!("Created new page: {}", path.display());
    Ok(())
}

async fn preview() -> anyhow::Result<()> {
    #[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
    struct PageId(String);

    impl PageId {
        fn from_str(s: &str) -> anyhow::Result<Self> {
            (s.len() == "00000000T000000Z".len()
                && s.chars().all(|c| matches!(c, '0'..='9' | 'T' | 'Z')))
            .then_some(Self(s.to_string()))
            .ok_or_else(|| anyhow::anyhow!("invalid ID format"))
        }
    }

    impl std::str::FromStr for PageId {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::from_str(s)
        }
    }

    impl std::fmt::Display for PageId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<'de> serde::de::Deserialize<'de> for PageId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'vi> serde::de::Visitor<'vi> for Visitor {
                type Value = PageId;

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
                    .map(PageId)
                    .ok_or_else(|| E::custom("invalid ID format"))
                }
            }

            deserializer.deserialize_string(Visitor)
        }
    }

    async fn get(
        axum::extract::State(state): axum::extract::State<std::sync::Arc<State>>,
        axum::extract::Path(id): axum::extract::Path<PageId>,
    ) -> Result<axum::response::Html<String>, axum::http::StatusCode> {
        if !state.page_metas.contains_key(&id) {
            return Err(axum::http::StatusCode::NOT_FOUND);
        }

        let id_str = id.to_string();
        let path = state.config.data_dir.join(&id_str).with_extension("md");
        let md = std::fs::read_to_string(path).map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
        let parser = pulldown_cmark::Parser::new(&md);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        #[derive(Debug, askama::Template)]
        #[template(path = "get.html")]
        struct Tmpl {
            html: String,
            id: String,
        }

        Ok(axum::response::Html(Tmpl { html, id: id_str }.to_string()))
    }

    async fn list(
        axum::extract::State(state): axum::extract::State<std::sync::Arc<State>>,
    ) -> Result<axum::response::Html<String>, axum::http::StatusCode> {
        #[derive(askama::Template)]
        #[template(path = "list.html")]
        struct Tmpl {
            page_metas: Vec<TmplPageMeta>,
        }
        struct TmplPageMeta {
            id: String,
            title: String,
        }

        let page_metas = state
            .page_metas
            .iter()
            .map(|(id, meta)| TmplPageMeta {
                id: id.to_string(),
                title: meta.title.clone().unwrap_or_default(),
            })
            .collect::<Vec<TmplPageMeta>>();

        Ok(axum::response::Html(Tmpl { page_metas }.to_string()))
    }

    let config = load_config().await?;

    // create index
    let read_dir = std::fs::read_dir(&config.data_dir).context("data dir not found")?;
    let mut page_ids = std::collections::BTreeSet::new();
    for dir_entry in read_dir {
        let dir_entry = dir_entry.context("dir_entry")?;
        let path_buf = dir_entry.path();
        let file_stem = path_buf.file_stem().context("file_stem")?;
        let page_id = file_stem.to_str().context("file_stem is not UTF-8")?;
        let page_id = PageId::from_str(page_id).context("invalid ID in data dir")?;
        page_ids.insert(page_id);
    }

    let mut page_metas = std::collections::BTreeMap::new();
    for page_id in &page_ids {
        let path = config
            .data_dir
            .join(page_id.to_string())
            .with_extension("md");
        let md = std::fs::read_to_string(path).context("read page")?;
        let page_meta = page_meta::PageMeta::from_markdown(&md);
        page_metas.insert(page_id.clone(), page_meta);
    }

    struct State {
        config: Config,
        page_metas: std::collections::BTreeMap<PageId, page_meta::PageMeta>,
    }

    let router = axum::Router::new()
        .route("/", axum::routing::get(list))
        .route("/{id}", axum::routing::get(get))
        .with_state(std::sync::Arc::new(State { config, page_metas }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

struct Config {
    data_dir: std::path::PathBuf,
}

async fn load_config() -> anyhow::Result<Config> {
    Ok(Config {
        data_dir: std::path::PathBuf::from("data"),
    })
}
