use axum::{
    extract::{Extension, Query, RawQuery},
    http::{header::LOCATION, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use crate::count::Count;

fn generate_impl(count: Option<Count>) -> Vec<String> {
    let mut generated = vec![];
    let count = count.unwrap_or_default();
    for _ in 0..usize::from(count) {
        let uuid = Uuid::new_v4();
        generated.push(uuid.to_string());
    }
    generated
}

pub fn generate(count: Option<Count>) -> anyhow::Result<()> {
    let generated = generate_impl(count);
    let message = generated.join("\n");
    print!("{}", message);
    Ok(())
}

async fn handler_root(
    Extension(state): Extension<Arc<State>>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    let location = format!(
        "{}/uuids.txt{}",
        state.base_path,
        query.map(|q| format!("?{}", q)).unwrap_or_default()
    );
    let mut header_map = HeaderMap::new();
    header_map.append(
        LOCATION,
        HeaderValue::from_str(&location).expect("state contains not ascii"),
    );
    (StatusCode::SEE_OTHER, header_map, ())
}

async fn handler_uuids(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let count = params.get("count").and_then(|s| s.parse::<Count>().ok());
    generate_impl(count).join("\n")
}

struct State {
    base_path: String,
}

pub async fn server() -> anyhow::Result<()> {
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
    .layer(Extension(Arc::new(state)));

    Ok(Server::bind(&addr)
        .serve(wrapped_router.into_make_service())
        .await?)
}
#[cfg(test)]
mod tests {
    use std::{collections::HashSet, convert::TryFrom};

    use super::*;

    #[test]
    fn generate_impl_test() -> anyhow::Result<()> {
        // count
        assert_eq!(generate_impl(None).len(), 1);
        assert_eq!(generate_impl(Some(Count::try_from(1_usize)?)).len(), 1);
        assert_eq!(generate_impl(Some(Count::try_from(100_usize)?)).len(), 100);

        // uniqueness
        assert_eq!(
            generate_impl(Some(Count::try_from(100_usize)?))
                .into_iter()
                .collect::<HashSet<_>>()
                .len(),
            100
        );

        Ok(())
    }
}
