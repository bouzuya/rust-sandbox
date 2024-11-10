#[derive(clap::Parser)]
struct Cli {
    mermaid_file_path: std::path::PathBuf,
}

async fn handle_socket(mut socket: axum::extract::ws::WebSocket, path_buf: std::path::PathBuf) {
    let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();
    let mut watcher = notify::recommended_watcher(tx).unwrap();
    let mut parent = path_buf.clone();
    parent.pop();
    notify::Watcher::watch(&mut watcher, &parent, notify::RecursiveMode::Recursive).unwrap();
    for res in rx {
        match res {
            Ok(event) => {
                // println!("event {:?}", event);
                match event.kind {
                    notify::EventKind::Modify(notify::event::ModifyKind::Data(_)) => {
                        println!("event data modify {:?}", event.paths);
                        if !event.paths.contains(&path_buf) {
                            continue;
                        }
                        if socket
                            .send(axum::extract::ws::Message::Text("Hello".to_owned()))
                            .await
                            .is_err()
                        {
                            // client disconnected
                            return;
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }
}

async fn ws_handler(
    ws: axum::extract::ws::WebSocketUpgrade,
    axum::extract::State(app_state): axum::extract::State<AppState>, // axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<std::net::SocketAddr>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, app_state.mermaid_file_path.clone()))
}

#[derive(Clone)]
struct AppState {
    mermaid_file_path: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    println!("{:?}", cli.mermaid_file_path);

    let router = axum::Router::new()
        .route_service(
            "/",
            tower_http::services::ServeFile::new("./public/index.html"),
        )
        .route_service(
            "/index.mmd",
            tower_http::services::ServeFile::new(cli.mermaid_file_path.clone()),
        )
        .route("/ws", axum::routing::any(ws_handler))
        .with_state(AppState {
            mermaid_file_path: cli.mermaid_file_path,
        });
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await?;
    let local_addr = listener.local_addr()?;
    println!("Listening on http://{:?}/", local_addr);
    tokio::spawn(async move { open::that(format!("http://localhost:{}", local_addr.port())) });
    axum::serve(listener, router).await?;
    Ok(())
}
