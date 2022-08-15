use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing, Router};

pub(crate) fn router() -> Router {
    Router::new().route("/users/:id", routing::get(users_show))
}

async fn users_show(Path(id): Path<String>) -> impl IntoResponse {
    // TODO: if the user is cached, return it; otherwise, enqueue the ID.
    (StatusCode::ACCEPTED, id)
}

#[cfg(test)]
mod tests {
    use hyper::StatusCode;

    use crate::router::tests::test_get_request;

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let (status, body) = test_get_request(router(), "/users/125962981").await?;
        assert_eq!(status, StatusCode::ACCEPTED);
        assert_eq!(body, r#"125962981"#);
        Ok(())
    }
}
