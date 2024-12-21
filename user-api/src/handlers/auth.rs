use axum::{
    extract::{Path, State},
    Json,
};

use crate::services::{AuthError, AuthInput, AuthOutput, AuthService};

#[derive(Debug, serde::Deserialize)]
struct PathParams {
    user_id: String,
}

#[derive(Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct RequestBody {
    user_secret: String,
}

impl std::fmt::Debug for RequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestBody")
            .field("user_secret", &"[FILTERED]")
            .finish()
    }
}

impl From<(PathParams, RequestBody)> for AuthInput {
    fn from(
        (PathParams { user_id }, RequestBody { user_secret }): (PathParams, RequestBody),
    ) -> Self {
        Self {
            user_id,
            user_secret,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ErrorResponse(axum::http::StatusCode, String);

impl From<AuthError> for ErrorResponse {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::InvalidUserId(_) => Self(
                axum::http::StatusCode::BAD_REQUEST,
                "user_id or user_secret is incorrect".to_owned(),
            ),
            AuthError::SecretNotMatch(_) => Self(
                axum::http::StatusCode::BAD_REQUEST,
                "user_id or user_secret is incorrect".to_owned(),
            ),
            AuthError::UserNotFound(_) => Self(
                axum::http::StatusCode::BAD_REQUEST,
                "user_id or user_secret is incorrect".to_owned(),
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

impl From<AuthOutput> for SuccessfulResponse {
    fn from(AuthOutput { user_id, user_name }: AuthOutput) -> Self {
        Self { user_id, user_name }
    }
}

impl axum::response::IntoResponse for SuccessfulResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, Json(self)).into_response()
    }
}

async fn handle<T: Clone + AuthService + Send + Sync + 'static>(
    State(state): State<T>,
    Path(path_params): Path<PathParams>,
    Json(request_body): Json<RequestBody>,
) -> Result<SuccessfulResponse, ErrorResponse> {
    let input = AuthInput::from((path_params, request_body));
    match state.auth(input).await {
        Err(error) => Err(ErrorResponse::from(error)),
        Ok(output) => Ok(SuccessfulResponse::from(output)),
    }
}

pub fn route<T: Clone + AuthService + Send + Sync + 'static>() -> axum::Router<T> {
    axum::Router::new().route("/users/:user_id/auth", axum::routing::post(handle::<T>))
}

#[cfg(test)]
mod tests {
    use crate::{
        handlers::tests::{send_request, ResponseExt as _},
        services::AuthError,
    };

    use super::*;

    #[derive(Clone)]
    struct MockAppState;

    #[axum::async_trait]
    impl AuthService for MockAppState {
        async fn auth(
            &self,
            AuthInput {
                user_id,
                user_secret: _,
            }: AuthInput,
        ) -> Result<AuthOutput, AuthError> {
            Ok(AuthOutput {
                user_id,
                user_name: "user_name1".to_owned(),
            })
        }
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let routes = route().with_state(MockAppState);
        let request = axum::http::Request::builder()
            .method("POST")
            .uri("/users/user_id1/auth")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(serde_json::to_vec(&RequestBody {
                user_secret: "user_secret1".to_owned(),
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
