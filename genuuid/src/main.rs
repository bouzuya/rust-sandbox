use axum::{
    extract::Extension,
    http::{header::LOCATION, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    AddExtensionLayer, Router, Server,
};
use std::{io, sync::Arc};
use structopt::{clap::Shell, StructOpt};
use uuid::Uuid;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "completion", about = "Prints the shell's completion script")]
    Completion {
        #[structopt(name = "SHELL", help = "the shell", possible_values = &Shell::variants())]
        shell: Shell,
    },
    #[structopt(name = "generate", about = "Generates UUID")]
    Generate,
    #[structopt(name = "server", about = "Runs server")]
    Server,
}

async fn handler_root(Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
    let mut header_map = HeaderMap::new();
    header_map.append(
        LOCATION,
        HeaderValue::from_str(&state.path("/uuids.txt")).expect("state contains not ascii"),
    );
    (StatusCode::SEE_OTHER, header_map, ())
}

async fn handler_uuids() -> impl IntoResponse {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}

struct State {
    base_path: String,
}

impl State {
    fn path(&self, s: &str) -> String {
        format!("{}{}", self.base_path, s)
    }
}

async fn server() -> anyhow::Result<()> {
    let base_path = std::env::var("BASE_PATH").unwrap_or_else(|_| "".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = "0.0.0.0";
    let addr = format!("{}:{}", host, port).parse()?;

    let state = State {
        base_path: base_path.clone(),
    };
    let router = Router::new()
        .route("/", get(handler_root))
        .route("/uuids.txt", get(handler_uuids));
    let wrapped_router = if base_path.is_empty() {
        router
    } else {
        Router::new()
            .route("/", get(handler_root))
            .nest(base_path.as_str(), router)
    }
    .layer(AddExtensionLayer::new(Arc::new(state)));

    Ok(Server::bind(&addr)
        .serve(wrapped_router.into_make_service())
        .await?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt.subcommand.unwrap_or(Subcommand::Generate) {
        Subcommand::Completion { shell } => {
            Opt::clap().gen_completions_to("genuuid", shell, &mut io::stdout());
            Ok(())
        }
        Subcommand::Generate => {
            let uuid = Uuid::new_v4();
            print!("{}", uuid);
            Ok(())
        }
        Subcommand::Server => server().await,
    }
}
