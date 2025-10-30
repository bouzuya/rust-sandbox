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
    let dir = config.data_dir;
    let page_id = page_id::PageId::new();
    let path = dir.join(page_id.to_string()).with_extension("md");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(&path, "")?;
    println!("Created new page: {}", path.display());
    Ok(())
}

async fn preview() -> anyhow::Result<()> {
    async fn get(
        axum::extract::State(state): axum::extract::State<std::sync::Arc<State>>,
        axum::extract::Path(id): axum::extract::Path<page_id::PageId>,
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
        let page_id = page_id::PageId::from_str(page_id).context("invalid ID in data dir")?;
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

struct Config {
    data_dir: std::path::PathBuf,
}

async fn load_config() -> anyhow::Result<Config> {
    Ok(Config {
        data_dir: std::path::PathBuf::from("data"),
    })
}
