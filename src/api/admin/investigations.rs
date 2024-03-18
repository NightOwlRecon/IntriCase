use axum::http::StatusCode;
use axum::routing::post;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
    Router,
};

use serde::Deserialize;
use serde_json::json;

use crate::{core::helpers::parse_form_date, core::investigations::Investigation, AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/create", post(create))
}

#[derive(Debug, Deserialize)]
pub struct CreateInvestigationRequest {
    internal_id: Option<String>,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    // TODO: deserialize/parse here
    date_of_birth: String,
    namus_id: Option<String>,
    // TODO: deserialize/parse here
    missing_since: String,
    synopsis: String,
}

// TODO: all of this needs cleanup
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateInvestigationRequest>,
) -> impl IntoResponse {
    let date_of_birth = parse_form_date(&req.date_of_birth);
    if date_of_birth.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid date of birth".to_string(),
        );
    }

    let missing_since = parse_form_date(&req.missing_since);
    if missing_since.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid date missing since".to_string(),
        );
    }

    let res = Investigation::create(
        State(state),
        req.internal_id,
        req.first_name,
        req.middle_name,
        req.last_name,
        date_of_birth.unwrap(),
        req.namus_id,
        missing_since.unwrap(),
        req.synopsis,
    )
    .await;

    if res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create investigation".to_string(),
        );
    }

    (StatusCode::OK, json!(res.unwrap()).to_string())
}
