use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing, Router};

pub(crate) fn router() -> Router {
    Router::new().merge(healthz()).merge(users_show())
}

fn healthz() -> Router {
    Router::new().route("/healthz", routing::get(healthz_handler))
}

async fn healthz_handler() -> impl IntoResponse {
    "OK"
}

fn users_show() -> Router {
    Router::new().route("/users/:id", routing::get(users_show_handler))
}

async fn users_show_handler(Path(id): Path<String>) -> impl IntoResponse {
    // TODO: if the user is cached, return it; otherwise, enqueue the ID.
    (StatusCode::ACCEPTED, id)
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn healthz_test() -> anyhow::Result<()> {
        let (status, body) = test_get_request(healthz(), "/healthz").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, r#"OK"#);
        Ok(())
    }

    #[tokio::test]
    async fn users_show_test() -> anyhow::Result<()> {
        let (status, body) = test_get_request(users_show(), "/users/125962981").await?;
        assert_eq!(status, StatusCode::ACCEPTED);
        assert_eq!(body, r#"125962981"#);
        Ok(())
    }

    async fn test_get_request(router: Router, uri: &str) -> anyhow::Result<(StatusCode, String)> {
        let request = Request::get(uri).body(Body::empty())?;
        let response = router.oneshot(request).await?;
        let status = response.status();
        let body_as_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let body_as_string = String::from_utf8(Vec::<u8>::from(body_as_bytes))?;
        Ok((status, body_as_string))
    }
}
