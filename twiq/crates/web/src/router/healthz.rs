use axum::{response::IntoResponse, routing, Router};

pub(crate) fn router() -> Router {
    Router::new().route("/healthz", routing::get(healthz))
}

async fn healthz() -> impl IntoResponse {
    "OK"
}

#[cfg(test)]
mod tests {
    use hyper::StatusCode;

    use crate::router::tests::test_get_request;

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let (status, body) = test_get_request(router(), "/healthz").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, r#"OK"#);
        Ok(())
    }
}
