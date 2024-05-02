use crate::core::investigations;
use crate::core::investigations::Investigation;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde_json::json;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all))
        .route("/:investigation_id", get(get_by_id))
}

pub async fn get_all(State(state): State<AppState>) -> impl IntoResponse {
    let invs = investigations::get_all(State(state.clone())).await;
    if invs.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string());
    }

    (StatusCode::OK, json!(invs.unwrap()).to_string())
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(investigation_id): Path<String>,
) -> impl IntoResponse {
    if let Ok(investigation_id) = Uuid::parse_str(&investigation_id) {
        if let Ok(investigation) =
            Investigation::get(State(state.clone()), &investigation_id.to_string(), true).await
        {
            return axum::Json(investigation).into_response();
        }
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
