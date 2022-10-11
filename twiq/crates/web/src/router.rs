mod healthz;
mod users_show;
mod worker;

use axum::Router;
use use_case::command::{create_user_request, request_user, send_user_request, update_user};

pub(crate) fn router<T>() -> Router
where
    T: create_user_request::Has
        + request_user::Has
        + send_user_request::Has
        + update_user::Has
        + Send
        + Sync
        + 'static,
{
    Router::new()
        .merge(healthz::router())
        .merge(users_show::router::<T>())
        .merge(worker::router::<T>())
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
