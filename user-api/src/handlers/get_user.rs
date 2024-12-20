use axum::{
    extract::{Path, State},
    Json,
};

use crate::services::{GetUserError, GetUserInput, GetUserOutput, GetUserService};

#[derive(Debug, serde::Deserialize)]
struct PathParams {
    user_id: String,
}

impl TryFrom<PathParams> for GetUserInput {
    type Error = ErrorResponse;

    fn try_from(PathParams { user_id }: PathParams) -> Result<Self, Self::Error> {
        Ok(GetUserInput { user_id })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ErrorResponse(axum::http::StatusCode, String);

impl From<GetUserError> for ErrorResponse {
    fn from(error: GetUserError) -> Self {
        match error {
            GetUserError::InvalidUserId(_) => Self(
                axum::http::StatusCode::BAD_REQUEST,
                "invalid user_id".to_owned(),
            ),
            GetUserError::UserNotFound(user_id) => Self(
                axum::http::StatusCode::NOT_FOUND,
                format!("user not found (id={})", user_id),
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

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct SuccessfulResponse {
    user_id: String,
    user_name: String,
}

impl From<GetUserOutput> for SuccessfulResponse {
    fn from(GetUserOutput { user_id, user_name }: GetUserOutput) -> Self {
        Self { user_id, user_name }
    }
}

impl axum::response::IntoResponse for SuccessfulResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, Json(self)).into_response()
    }
}

async fn handle<T: Clone + GetUserService + Send + Sync + 'static>(
    Path(path_params): Path<PathParams>,
    State(state): State<T>,
) -> Result<SuccessfulResponse, ErrorResponse> {
    let input = GetUserInput::try_from(path_params)?;
    match state.get_user(input).await {
        Err(error) => Err(ErrorResponse::from(error)),
        Ok(output) => Ok(SuccessfulResponse::from(output)),
    }
}

pub fn route<T: Clone + GetUserService + Send + Sync + 'static>() -> axum::Router<T> {
    axum::Router::new().route("/users/:user_id", axum::routing::get(handle::<T>))
}

#[cfg(test)]
mod tests {
    use crate::handlers::tests::{send_request, ResponseExt as _};

    use super::*;

    #[derive(Clone)]
    struct MockAppState;

    #[axum::async_trait]
    impl GetUserService for MockAppState {
        async fn get_user(
            &self,
            GetUserInput { user_id }: GetUserInput,
        ) -> Result<GetUserOutput, GetUserError> {
            Ok(GetUserOutput {
                user_id,
                user_name: "user_name1".to_owned(),
            })
        }
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let routes = route().with_state(MockAppState);
        let request = axum::http::Request::builder()
            .method("GET")
            .uri("/users/user_id1")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::empty())?;
        let response = send_request(routes, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            response.into_body_as_json::<SuccessfulResponse>().await?,
            SuccessfulResponse {
                user_id: "user_id1".to_owned(),
                user_name: "user_name1".to_owned(),
            }
        );
        Ok(())
    }
}
