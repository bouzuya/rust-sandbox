mod healthz;
mod users_show;

use axum::Router;

pub(crate) fn router() -> Router {
    Router::new()
        .merge(healthz::router())
        .merge(users_show::router())
}

#[cfg(test)]
mod tests {
    use hyper::{Body, Request, StatusCode};
    use tower::ServiceExt;

    use super::*;

    // test helper
    pub(crate) async fn test_get_request(
        router: Router,
        uri: &str,
    ) -> anyhow::Result<(StatusCode, String)> {
        let request = Request::get(uri).body(Body::empty())?;
        let response = router.oneshot(request).await?;
        let status = response.status();
        let body_as_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let body_as_string = String::from_utf8(Vec::<u8>::from(body_as_bytes))?;
        Ok((status, body_as_string))
    }
}
