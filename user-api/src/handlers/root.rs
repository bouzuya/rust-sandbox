use crate::AppState;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct ResponseBody {
    message: String,
}

#[tracing::instrument]
async fn handle() -> axum::response::Json<ResponseBody> {
    axum::response::Json(ResponseBody {
        message: "OK".to_owned(),
    })
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/", axum::routing::get(handle))
}

#[cfg(test)]
mod tests {
    use crate::handlers::tests::{send_request, ResponseExt as _};

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let routes = route().with_state(AppState::default());
        let request = axum::http::Request::builder()
            .uri("/")
            .body(axum::body::Body::empty())?;
        let response = send_request(routes, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            response.into_body_as_json::<ResponseBody>().await?,
            ResponseBody {
                message: "OK".to_owned()
            }
        );
        Ok(())
    }
}
