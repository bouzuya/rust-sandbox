use crate::AppState;

mod create_user;
mod delete_user;
mod root;
mod update_user;

pub fn route() -> axum::Router<AppState> {
    axum::Router::new()
        .merge(create_user::route())
        .merge(delete_user::route())
        .merge(root::route())
        .merge(update_user::route())
}

#[cfg(test)]
mod tests {
    #[axum::async_trait]
    pub(crate) trait ResponseExt {
        async fn into_body_string(self) -> anyhow::Result<String>;
        async fn into_body_as_json<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T>;
    }

    #[axum::async_trait]
    impl ResponseExt for axum::http::Response<axum::body::Body> {
        async fn into_body_string(self) -> anyhow::Result<String> {
            let body = axum::body::to_bytes(self.into_body(), usize::MAX).await?;
            Ok(String::from_utf8(body.to_vec())?)
        }
        async fn into_body_as_json<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T> {
            Ok(serde_json::from_str(&self.into_body_string().await?)?)
        }
    }

    pub(crate) async fn send_request(
        app: axum::Router,
        request: axum::http::Request<axum::body::Body>,
    ) -> anyhow::Result<axum::response::Response<axum::body::Body>> {
        Ok(tower::ServiceExt::oneshot(app, request).await?)
    }
}
