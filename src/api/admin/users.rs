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
        return axum::Json(users).into_response();
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

pub async fn invite(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> impl IntoResponse {
    if let Ok(user) = User::create(&state.db, &request.email).await {
        return axum::Json(user).into_response();
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
