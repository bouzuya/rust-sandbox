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
        links: Vec<String>,
        rev_links: Vec<String>,
        title: String,
    }

    impl axum::response::IntoResponse for GetResponse {
        fn into_response(self) -> axum::response::Response {
            let body = self.to_string();
            axum::response::Html(body).into_response()
        }
    }

    async fn get(
        axum::extract::State(state): axum::extract::State<std::sync::Arc<std::sync::Mutex<State>>>,
        axum::extract::Path(id): axum::extract::Path<page_id::PageId>,
    ) -> Result<GetResponse, axum::http::StatusCode> {
        // FIXME: unwrap
        let state = state.lock().unwrap();
        let page_meta = state
            .page_metas
            .get(&id)
            .ok_or(axum::http::StatusCode::NOT_FOUND)?;

        let path = page_path(&state.config, &id);
        let md = std::fs::read_to_string(path).map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
        let parser = pulldown_cmark::Parser::new_with_broken_link_callback(
            &md,
            pulldown_cmark::Options::empty(),
            Some(|broken_link: pulldown_cmark::BrokenLink<'_>| {
                match <page_id::PageId as std::str::FromStr>::from_str(&broken_link.reference) {
                    Err(_) => None,
                    Ok(page_id) => Some((
                        pulldown_cmark::CowStr::Boxed(page_id.to_string().into_boxed_str()),
                        pulldown_cmark::CowStr::Boxed(format!("/{page_id}").into_boxed_str()),
                    )),
                }
            }),
        );
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        Ok(GetResponse {
            html,
            id: id.to_string(),
            links: page_meta
                .links
                .clone()
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>(),
            rev_links: state
                .rev_index
                .get(&id)
                .map(|set| {
                    set.iter()
                        .cloned()
                        .map(|id| id.to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default(),
            title: page_meta.title.clone().unwrap_or_default(),
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
        axum::extract::State(state): axum::extract::State<std::sync::Arc<std::sync::Mutex<State>>>,
    ) -> Result<ListResponse, axum::http::StatusCode> {
        // FIXME: unwrap
        let state = state.lock().unwrap();
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

    let mut rev_index = std::collections::BTreeMap::new();
    for (page_id, page_meta) in &page_metas {
        for linked_page_id in &page_meta.links {
            rev_index
                .entry(linked_page_id.clone())
                .or_insert_with(std::collections::BTreeSet::new)
                .insert(page_id.clone());
        }
    }

    struct State {
        config: Config,
        page_metas: std::collections::BTreeMap<page_id::PageId, page_meta::PageMeta>,
        rev_index: std::collections::BTreeMap<
            page_id::PageId,
            std::collections::BTreeSet<page_id::PageId>,
        >,
    }

    let watch_dir = config.data_dir.clone();
    let state = std::sync::Arc::new(std::sync::Mutex::new(State {
        config,
        page_metas,
        rev_index,
    }));

    // run watcher
    fn update_page_meta(
        state: std::sync::Arc<std::sync::Mutex<State>>,
        path: &std::path::Path,
    ) -> anyhow::Result<()> {
        let mut state = state.lock().unwrap();

        let file_stem = path.file_stem().context("file_stem")?;
        let page_id = file_stem.to_str().context("file_stem is not UTF-8")?;
        let page_id = page_id::PageId::from_str(page_id).context("invalid ID in data dir")?;

        if !path.exists() {
            let old_page_meta = state.page_metas.get(&page_id).cloned();
            match old_page_meta {
                Some(old_page_meta) => {
                    // remove old links from rev_index
                    for linked_page_id in &old_page_meta.links {
                        if let Some(set) = state.rev_index.get_mut(linked_page_id) {
                            set.remove(&page_id);
                        }
                    }
                }
                None => {
                    // do nothing
                }
            }
            return Ok(());
        }

        let md = std::fs::read_to_string(path).context("read page")?;
        let new_page_meta = page_meta::PageMeta::from_markdown(&md);

        let old_page_meta = state.page_metas.get(&page_id).cloned();
        match old_page_meta {
            Some(old_page_meta) => {
                // remove old links from rev_index
                for linked_page_id in &old_page_meta.links {
                    if let Some(set) = state.rev_index.get_mut(linked_page_id) {
                        set.remove(&page_id);
                    }
                }
            }
            None => {
                // do nothing
            }
        }

        for linked_page_id in &new_page_meta.links {
            state
                .rev_index
                .entry(linked_page_id.clone())
                .or_insert_with(std::collections::BTreeSet::new)
                .insert(page_id.clone());
        }

        state
            .page_metas
            .insert(page_id.clone(), new_page_meta.clone());

        Ok(())
    }

    let state_for_watcher = state.clone();
    tokio::spawn(async move {
        let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();
        // FIXME: unwrap
        let mut watcher = notify::recommended_watcher(tx).unwrap();
        // FIXME: unwrap
        notify::Watcher::watch(&mut watcher, &watch_dir, notify::RecursiveMode::Recursive).unwrap();
        for res in rx {
            match res {
                Ok(event) => {
                    match event.kind {
                        notify::EventKind::Any
                        | notify::EventKind::Access(_)
                        | notify::EventKind::Other => {
                            // do nothing
                        }
                        notify::EventKind::Create(_)
                        | notify::EventKind::Modify(_)
                        | notify::EventKind::Remove(_) => {
                            for path in event.paths {
                                // FIXME: unwrap
                                update_page_meta(state_for_watcher.clone(), &path).unwrap();
                            }
                        }
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    let router = axum::Router::new()
        .route("/", axum::routing::get(list))
        .route("/{id}", axum::routing::get(get))
        .with_state(state);
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
