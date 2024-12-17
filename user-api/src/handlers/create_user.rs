use axum::{extract::State, Json};

use crate::services::{CreateUserError, CreateUserInput, CreateUserOutput, CreateUserService};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct RequestBody {
    name: String,
}

impl TryFrom<RequestBody> for CreateUserInput {
    type Error = ErrorResponse;

    fn try_from(RequestBody { name }: RequestBody) -> Result<Self, Self::Error> {
        if name.is_empty() {
            return Err(ErrorResponse(
                axum::http::StatusCode::BAD_REQUEST,
                "name is empty".to_owned(),
            ));
        }
        Ok(Self { name })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ErrorResponse(axum::http::StatusCode, String);

impl From<CreateUserError> for ErrorResponse {
    fn from(error: CreateUserError) -> Self {
        match error {
            CreateUserError::NewUser(_) => Self(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "new_user".to_owned(),
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
    user_secret: String,
}

impl From<CreateUserOutput> for SuccessfulResponse {
    fn from(
        CreateUserOutput {
            user_id,
            user_name,
            user_secret,
        }: CreateUserOutput,
    ) -> Self {
        Self {
            user_id,
            user_name,
            user_secret,
        }
    }
}

impl axum::response::IntoResponse for SuccessfulResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::CREATED, Json(self)).into_response()
    }
}

async fn handle<T: Clone + CreateUserService + Send + Sync + 'static>(
    State(state): State<T>,
    Json(request_body): Json<RequestBody>,
) -> Result<SuccessfulResponse, ErrorResponse> {
    let input = CreateUserInput::try_from(request_body)?;
    match state.create_user(input).await {
        Err(error) => Err(ErrorResponse::from(error)),
        Ok(output) => Ok(SuccessfulResponse::from(output)),
    }
}

pub fn route<T: Clone + CreateUserService + Send + Sync + 'static>() -> axum::Router<T> {
    axum::Router::new().route("/users", axum::routing::post(handle::<T>))
}

#[cfg(test)]
mod tests {
    use crate::{
        handlers::tests::{send_request, ResponseExt as _},
        services::CreateUserError,
    };

    use super::*;

    #[derive(Clone)]
    struct MockAppState;

    #[axum::async_trait]
    impl CreateUserService for MockAppState {
        async fn create_user(
            &self,
            CreateUserInput { name }: CreateUserInput,
        ) -> Result<CreateUserOutput, CreateUserError> {
            Ok(CreateUserOutput {
                user_id: "user_id1".to_owned(),
                user_name: name,
                user_secret: "user_secret1".to_owned(),
            })
        }
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let routes = route().with_state(MockAppState);
        let request = axum::http::Request::builder()
            .method("POST")
            .uri("/users")
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
                user_secret: "user_secret1".to_owned()
            }
        );
        Ok(())
    }
}
