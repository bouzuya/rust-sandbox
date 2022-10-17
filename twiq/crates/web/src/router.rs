mod healthz;
mod users_show;
mod worker;

use ::worker::command::{create_user_request, send_user_request, update_query_user, update_user};
use axum::Router;
use query_handler::user_store::HasUserStore;
use use_case::command::request_user;

pub(crate) fn router<T>() -> Router
where
    T: create_user_request::Has
        + request_user::Has
        + send_user_request::Has
        + update_query_user::Has
        + update_user::Has
        + HasUserStore
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
