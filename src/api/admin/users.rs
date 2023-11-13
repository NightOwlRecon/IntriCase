use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Router};
use axum_extra::extract::Form;
use serde::Deserialize;

use crate::{core::users::User, AppState};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    email: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/invite", post(invite))
}

pub async fn invite(
    State(state): State<AppState>,
    Form(request): Form<CreateUserRequest>,
) -> impl IntoResponse {
    let user = User::create(&state.db, &request.email).await;
    match user {
        Ok(_user) => StatusCode::OK,
        Err(_e) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
