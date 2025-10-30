mod page_id;
mod page_meta;

use std::str::FromStr as _;

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
    let page_id = page_id::PageId::new();
    let path = page_path(&config, &page_id);
    std::fs::create_dir_all(path.parent().context("invalid path")?)?;
    std::fs::write(&path, "")?;
    println!("Created new page: {}", path.display());
    Ok(())
}

async fn preview() -> anyhow::Result<()> {
    #[derive(Debug, askama::Template)]
    #[template(path = "get.html")]
    struct GetResponse {
        html: String,
        id: String,
    }

    impl axum::response::IntoResponse for GetResponse {
        fn into_response(self) -> axum::response::Response {
            let body = self.to_string();
            axum::response::Html(body).into_response()
        }
    }

    async fn get(
        axum::extract::State(state): axum::extract::State<std::sync::Arc<State>>,
        axum::extract::Path(id): axum::extract::Path<page_id::PageId>,
    ) -> Result<GetResponse, axum::http::StatusCode> {
        if !state.page_metas.contains_key(&id) {
            return Err(axum::http::StatusCode::NOT_FOUND);
        }

        let path = page_path(&state.config, &id);
        let md = std::fs::read_to_string(path).map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
        let parser = pulldown_cmark::Parser::new(&md);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        Ok(GetResponse {
            html,
            id: id.to_string(),
        })
    }

    #[derive(askama::Template)]
    #[template(path = "list.html")]
    struct ListResponse {
        page_metas: Vec<ListResponsePageMeta>,
    }

    impl axum::response::IntoResponse for ListResponse {
        fn into_response(self) -> axum::response::Response {
            let body = self.to_string();
            axum::response::Html(body).into_response()
        }
    }

    struct ListResponsePageMeta {
        id: String,
        title: String,
    }

    async fn list(
        axum::extract::State(state): axum::extract::State<std::sync::Arc<State>>,
    ) -> Result<ListResponse, axum::http::StatusCode> {
        let page_metas = state
            .page_metas
            .iter()
            .map(|(id, meta)| ListResponsePageMeta {
                id: id.to_string(),
                title: meta.title.clone().unwrap_or_default(),
            })
            .collect::<Vec<ListResponsePageMeta>>();

        Ok(ListResponse { page_metas })
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
        let page_id = page_id::PageId::from_str(page_id).context("invalid ID in data dir")?;
        page_ids.insert(page_id);
    }

    let mut page_metas = std::collections::BTreeMap::new();
    for page_id in &page_ids {
        let path = page_path(&config, page_id);
        let md = std::fs::read_to_string(path).context("read page")?;
        let page_meta = page_meta::PageMeta::from_markdown(&md);
        page_metas.insert(page_id.clone(), page_meta);
    }

    struct State {
        config: Config,
        page_metas: std::collections::BTreeMap<page_id::PageId, page_meta::PageMeta>,
    }

    let router = axum::Router::new()
        .route("/", axum::routing::get(list))
        .route("/{id}", axum::routing::get(get))
        .with_state(std::sync::Arc::new(State { config, page_metas }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

fn page_path(config: &Config, page_id: &page_id::PageId) -> std::path::PathBuf {
    config
        .data_dir
        .join(page_id.to_string())
        .with_extension("md")
}

struct Config {
    data_dir: std::path::PathBuf,
}

async fn load_config() -> anyhow::Result<Config> {
    Ok(Config {
        data_dir: std::path::PathBuf::from("data"),
    })
}
