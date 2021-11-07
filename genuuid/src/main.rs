use axum::{
    http::{header::LOCATION, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use std::io;
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

async fn handler_root() -> impl IntoResponse {
    let mut header_map = HeaderMap::new();
    header_map.append(LOCATION, HeaderValue::from_static("/uuids.txt"));
    (StatusCode::SEE_OTHER, header_map, ())
}

async fn handler_uuids() -> impl IntoResponse {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}

async fn server() -> anyhow::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = "0.0.0.0";
    let addr = format!("{}:{}", host, port).parse()?;
    let router = Router::new()
        .route("/", get(handler_root))
        .route("/uuids.txt", get(handler_uuids));
    Ok(Server::bind(&addr)
        .serve(router.into_make_service())
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
