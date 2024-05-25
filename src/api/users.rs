use crate::core::users;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        // by default we only return enabled users
        .route("/", get(get_enabled))
        .route("/all", get(get_all))
}

// TODO: this is duplicated in the `admin` module - check fields that need to be masked for non-admins
pub async fn get_enabled(State(state): State<AppState>) -> impl IntoResponse {
    if let Ok(users) = users::get_enabled(&state.db).await {
        return axum::Json(users).into_response();
    }
    axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

pub async fn get_all(State(state): State<AppState>) -> impl IntoResponse {
    if let Ok(users) = users::get_all(&state.db).await {
        return axum::Json(users).into_response();
    }
    axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
