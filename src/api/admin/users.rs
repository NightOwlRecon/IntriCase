use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    core::users::{self, User},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    email: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/list", get(list))
        .route("/invite", post(invite))
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    if let Ok(users) = users::get_all(&state.db).await {
        return (StatusCode::OK, json!(users).to_string());
    }
    (StatusCode::INTERNAL_SERVER_ERROR, String::new())
}

pub async fn invite(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let user = User::create(&state.db, &request.email).await;
    match user {
        // we return the (serializable) user details here to allow the admin to see the OTP
        Ok(user) => (StatusCode::OK, json!(user).to_string()),
        Err(_e) => (StatusCode::INTERNAL_SERVER_ERROR, String::new()),
    }
}
