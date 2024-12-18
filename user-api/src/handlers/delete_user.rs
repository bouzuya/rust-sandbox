use axum::{
    extract::{Path, State},
    Json,
};

use crate::services::{DeleteUserError, DeleteUserInput, DeleteUserOutput, DeleteUserService};

#[derive(Debug, serde::Deserialize)]
struct PathParams {
    user_id: String,
}

impl TryFrom<PathParams> for DeleteUserInput {
    type Error = ErrorResponse;

    fn try_from(PathParams { user_id }: PathParams) -> Result<Self, Self::Error> {
        Ok(DeleteUserInput { user_id })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ErrorResponse(axum::http::StatusCode, String);

impl From<DeleteUserError> for ErrorResponse {
    fn from(error: DeleteUserError) -> Self {
        match error {
            DeleteUserError::InvalidUserId(_) => Self(
                axum::http::StatusCode::BAD_REQUEST,
                "invalid user_id".to_owned(),
            ),
        }
    }
}

impl axum::response::IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        #[derive(serde::Serialize)]
        struct ResponseBody {
            message: String,
        }
        (self.0, Json(ResponseBody { message: self.1 })).into_response()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct SuccessfulResponse;

impl From<DeleteUserOutput> for SuccessfulResponse {
    fn from(_: DeleteUserOutput) -> Self {
        Self
    }
}

impl axum::response::IntoResponse for SuccessfulResponse {
    fn into_response(self) -> axum::response::Response {
        axum::http::StatusCode::NO_CONTENT.into_response()
    }
}

async fn handle<T: Clone + DeleteUserService + Send + Sync + 'static>(
    Path(path_params): Path<PathParams>,
    State(state): State<T>,
) -> Result<SuccessfulResponse, ErrorResponse> {
    let input = DeleteUserInput::try_from(path_params)?;
    match state.delete_user(input).await {
        Err(error) => Err(ErrorResponse::from(error)),
        Ok(output) => Ok(SuccessfulResponse::from(output)),
    }
}

pub fn route<T: Clone + DeleteUserService + Send + Sync + 'static>() -> axum::Router<T> {
    axum::Router::new().route("/users/:user_id", axum::routing::delete(handle::<T>))
}

#[cfg(test)]
mod tests {
    use crate::handlers::tests::send_request;

    use super::*;

    #[derive(Clone)]
    struct MockAppState;

    #[axum::async_trait]
    impl DeleteUserService for MockAppState {
        async fn delete_user(
            &self,
            DeleteUserInput { user_id: _ }: DeleteUserInput,
        ) -> Result<DeleteUserOutput, DeleteUserError> {
            Ok(DeleteUserOutput)
        }
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let routes = route().with_state(MockAppState);
        let request = axum::http::Request::builder()
            .method("DELETE")
            .uri("/users/user_id1")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::empty())?;
        let response = send_request(routes, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::NO_CONTENT);
        Ok(())
    }
}
