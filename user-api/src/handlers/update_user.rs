use axum::{
    extract::{Path, State},
    Json,
};

use crate::services::{UpdateUserError, UpdateUserInput, UpdateUserOutput, UpdateUserService};

#[derive(Debug, serde::Deserialize)]
struct PathParams {
    user_id: String,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct RequestBody {
    name: String,
}

#[derive(Debug, Eq, PartialEq)]
struct ErrorResponse(axum::http::StatusCode, String);

impl From<UpdateUserError> for ErrorResponse {
    fn from(error: UpdateUserError) -> Self {
        match error {
            UpdateUserError::InvalidUserId(_) => Self(
                axum::http::StatusCode::BAD_REQUEST,
                "user_id is invalid".to_owned(),
            ),
            UpdateUserError::UserNotFound(user_id) => Self(
                axum::http::StatusCode::NOT_FOUND,
                format!("user not found (id={})", user_id),
            ),
            UpdateUserError::UserUpdate(_) => Self(
                axum::http::StatusCode::BAD_REQUEST,
                "update_user".to_owned(),
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

impl From<UpdateUserOutput> for SuccessfulResponse {
    fn from(UpdateUserOutput { user_id, user_name }: UpdateUserOutput) -> Self {
        Self { user_id, user_name }
    }
}

impl axum::response::IntoResponse for SuccessfulResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, Json(self)).into_response()
    }
}

async fn handle<T: Clone + UpdateUserService + Send + Sync + 'static>(
    State(state): State<T>,
    Path(path_params): Path<PathParams>,
    Json(request_body): Json<RequestBody>,
) -> Result<SuccessfulResponse, ErrorResponse> {
    if request_body.name.is_empty() {
        return Err(ErrorResponse(
            axum::http::StatusCode::BAD_REQUEST,
            "name is empty".to_owned(),
        ));
    }
    let input = UpdateUserInput {
        name: request_body.name,
        user_id: path_params.user_id,
    };
    match state.update_user(input).await {
        Err(error) => Err(ErrorResponse::from(error)),
        Ok(output) => Ok(SuccessfulResponse::from(output)),
    }
}

pub fn route<T: Clone + UpdateUserService + Send + Sync + 'static>() -> axum::Router<T> {
    axum::Router::new().route("/users/:user_id", axum::routing::patch(handle::<T>))
}

#[cfg(test)]
mod tests {
    use crate::{
        handlers::tests::{send_request, ResponseExt as _},
        services::UpdateUserError,
    };

    use super::*;

    #[derive(Clone)]
    struct MockAppState;

    #[axum::async_trait]
    impl UpdateUserService for MockAppState {
        async fn update_user(
            &self,
            UpdateUserInput { name, user_id }: UpdateUserInput,
        ) -> Result<UpdateUserOutput, UpdateUserError> {
            Ok(UpdateUserOutput {
                user_id,
                user_name: name,
            })
        }
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let routes = route().with_state(MockAppState);
        let request = axum::http::Request::builder()
            .method("PATCH")
            .uri("/users/user_id1")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(serde_json::to_vec(&RequestBody {
                name: "user_name1".to_owned(),
            })?))?;
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
