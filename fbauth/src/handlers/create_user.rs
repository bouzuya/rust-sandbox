use crate::user::User;
use crate::AppState;
use axum::{extract::State, routing::post, Json};

#[derive(serde::Serialize)]
struct CreateUserResponse {
    user_id: String,
    user_secret: String,
}

async fn create_user(State(app_state): State<AppState>) -> Json<CreateUserResponse> {
    let mut users = app_state.users.lock().unwrap();
    let (user, raw) = User::new().expect("FIXME");
    users.insert(user.id, user.clone());
    Json(CreateUserResponse {
        user_id: user.id.to_string(),
        user_secret: raw,
    })
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/users", post(create_user))
}
