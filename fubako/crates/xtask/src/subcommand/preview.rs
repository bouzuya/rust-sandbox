use anyhow::Context as _;

pub(super) async fn execute() -> anyhow::Result<()> {
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
        axum::extract::Path(page_id): axum::extract::Path<crate::page_id::PageId>,
    ) -> Result<GetResponse, axum::http::StatusCode> {
        let state = state.lock().map_err(|_| axum::http::StatusCode::CONFLICT)?;
        let page_meta = state
            .page_metas
            .get(&page_id)
            .ok_or(axum::http::StatusCode::NOT_FOUND)?;

        let html = crate::page_io::PageIo::read_page_content(&state.config, &page_id)
            .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

        Ok(GetResponse {
            html,
            id: page_id.to_string(),
            links: page_meta
                .links
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>(),
            rev_links: state
                .rev_index
                .get(&page_id)
                .map(|set| set.iter().map(|id| id.to_string()).collect::<Vec<String>>())
                .unwrap_or_default(),
            title: page_meta.title.clone().unwrap_or_default(),
        })
    }

    #[derive(askama::Template)]
    #[template(path = "list.html")]
    struct ListResponse {
        page_metas: Vec<ListResponsePageMeta>,
        q: String,
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

    #[derive(serde::Deserialize)]
    struct ListRequestQuery {
        q: Option<String>,
    }

    async fn list(
        axum::extract::State(state): axum::extract::State<std::sync::Arc<std::sync::Mutex<State>>>,
        axum::extract::Query(ListRequestQuery { q }): axum::extract::Query<ListRequestQuery>,
    ) -> Result<ListResponse, axum::http::StatusCode> {
        let q = q.unwrap_or_default().trim().to_owned();
        let state = state.lock().map_err(|_| axum::http::StatusCode::CONFLICT)?;
        let config = &state.config;
        let page_metas = state
            .page_metas
            .iter()
            .filter(|(page_id, _page_meta)| {
                q.is_empty() || {
                    crate::page_io::PageIo::read_page_content(config, page_id)
                        .is_ok_and(|content| content.contains(&q))
                }
            })
            .map(|(id, meta)| ListResponsePageMeta {
                id: id.to_string(),
                title: meta.title.clone().unwrap_or_default(),
            })
            .collect::<Vec<ListResponsePageMeta>>();
        Ok(ListResponse { page_metas, q })
    }

    let config = crate::config::Config::load().await?;

    // create index
    let page_ids = crate::page_io::PageIo::read_page_ids(&config)?;

    let mut page_metas = std::collections::BTreeMap::new();
    for page_id in &page_ids {
        let page_meta = crate::page_io::PageIo::read_page_meta(&config, page_id)?;
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
        config: crate::config::Config,
        page_metas: std::collections::BTreeMap<crate::page_id::PageId, crate::page_meta::PageMeta>,
        rev_index: std::collections::BTreeMap<
            crate::page_id::PageId,
            std::collections::BTreeSet<crate::page_id::PageId>,
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
        let mut state = state.lock().map_err(|_| anyhow::anyhow!("locking state"))?;

        let page_id = crate::page_io::PageIo::page_id(path)?;

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

        let new_page_meta = crate::page_io::PageIo::read_page_meta(&state.config, &page_id)?;

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

    async fn new_watcher(
        state_for_watcher: std::sync::Arc<std::sync::Mutex<State>>,
        watch_dir: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();
        let mut watcher = notify::recommended_watcher(tx).context("create watcher")?;
        notify::Watcher::watch(&mut watcher, &watch_dir, notify::RecursiveMode::Recursive)
            .context("watch dir")?;
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
                                update_page_meta(state_for_watcher.clone(), &path)
                                    .context("update page meta")?;
                            }
                        }
                    }
                }
                Err(e) => anyhow::bail!("watch error: {:?}", e),
            }
        }
        Ok(())
    }
    tokio::spawn(new_watcher(state.clone(), watch_dir));

    let router = axum::Router::new()
        .route("/", axum::routing::get(list))
        .route("/{id}", axum::routing::get(get))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
