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

fn generate(count: Option<usize>) -> Vec<String> {
    let mut generated = vec![];
    let count = count.unwrap_or(1).max(1).min(100);
    for _ in 0..count {
        let uuid = Uuid::new_v4();
        generated.push(uuid.to_string());
    }
    generated
}

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
    Generate {
        #[structopt(long = "count", help = "the count")]
        count: Option<usize>,
    },
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
    match opt
        .subcommand
        .unwrap_or(Subcommand::Generate { count: None })
    {
        Subcommand::Completion { shell } => {
            Opt::clap().gen_completions_to("genuuid", shell, &mut io::stdout());
            Ok(())
        }
        Subcommand::Generate { count } => {
            let generated = generate(count);
            let message = generated.join("\n");
            print!("{}", message);
            Ok(())
        }
        Subcommand::Server => server().await,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn generate_test() {
        // count
        assert_eq!(generate(None).len(), 1);
        assert_eq!(generate(Some(0)).len(), 1);
        assert_eq!(generate(Some(1)).len(), 1);
        assert_eq!(generate(Some(2)).len(), 2);
        assert_eq!(generate(Some(100)).len(), 100);
        assert_eq!(generate(Some(101)).len(), 100);

        // uniqueness
        assert_eq!(
            generate(Some(100))
                .into_iter()
                .collect::<HashSet<_>>()
                .len(),
            100
        );
    }
}
