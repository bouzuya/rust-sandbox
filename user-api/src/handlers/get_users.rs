use axum::{extract::State, Json};

use crate::services::{
    GetUsersError, GetUsersInput, GetUsersOutput, GetUsersOutputItem, GetUsersService,
};

#[derive(Debug, Eq, PartialEq)]
struct ErrorResponse;

impl From<GetUsersError> for ErrorResponse {
    fn from(_: GetUsersError) -> Self {
        Self
    }
}

impl axum::response::IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct SuccessfulResponse {
    users: Vec<SuccessfulResponseUser>,
}

impl From<GetUsersOutput> for SuccessfulResponse {
    fn from(GetUsersOutput { users }: GetUsersOutput) -> Self {
        Self {
            users: users
                .into_iter()
                .map(SuccessfulResponseUser::from)
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct SuccessfulResponseUser {
    user_id: String,
    user_name: String,
}

impl From<GetUsersOutputItem> for SuccessfulResponseUser {
    fn from(GetUsersOutputItem { user_id, user_name }: GetUsersOutputItem) -> Self {
        Self { user_id, user_name }
    }
}

impl axum::response::IntoResponse for SuccessfulResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, Json(self)).into_response()
    }
}

async fn handle<T: Clone + GetUsersService + Send + Sync + 'static>(
    State(state): State<T>,
) -> Result<SuccessfulResponse, ErrorResponse> {
    match state.get_users(GetUsersInput).await {
        Err(error) => Err(ErrorResponse::from(error)),
        Ok(output) => Ok(SuccessfulResponse::from(output)),
    }
}

pub fn route<T: Clone + GetUsersService + Send + Sync + 'static>() -> axum::Router<T> {
    axum::Router::new().route("/users", axum::routing::get(handle::<T>))
}

#[cfg(test)]
mod tests {
    use crate::handlers::tests::{send_request, ResponseExt as _};

    use super::*;

    #[derive(Clone)]
    struct MockAppState;

    #[axum::async_trait]
    impl GetUsersService for MockAppState {
        async fn get_users(&self, _: GetUsersInput) -> Result<GetUsersOutput, GetUsersError> {
            Ok(GetUsersOutput {
                users: vec![GetUsersOutputItem {
                    user_id: "user_id1".to_owned(),
                    user_name: "user_name1".to_owned(),
                }],
            })
        }
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let routes = route().with_state(MockAppState);
        let request = axum::http::Request::builder()
            .method("GET")
            .uri("/users")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::empty())?;
        let response = send_request(routes, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            response.into_body_as_json::<SuccessfulResponse>().await?,
            SuccessfulResponse {
                users: vec![SuccessfulResponseUser {
                    user_id: "user_id1".to_owned(),
                    user_name: "user_name1".to_owned(),
                }]
            }
        );
        Ok(())
    }
}
