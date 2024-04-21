use axum::http::StatusCode;
use axum::routing::post;
use axum::Extension;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
    Router,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::core::investigations::ActionItem;
use crate::core::users::User;
use crate::{core::helpers::parse_form_date, core::investigations::Investigation, AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/create", post(create))
}

#[derive(Debug, Deserialize)]
pub struct CreateInvestigationDetails {
    pub internal_id: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub namus_id: Option<String>,
    pub missing_since: NaiveDate,
    pub synopsis: String,
    pub questions: Vec<CreateQuestionDetails>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuestionDetails {
    pub pretty_id: String,
    pub summary: String,
    pub details: Option<String>,
    pub investigation: Uuid,
    pub outcome: Option<String>,
    pub status: String,
    pub action_items: Vec<CreateActionItemDetails>,
}

#[derive(Debug, Deserialize)]
pub struct CreateActionItemDetails {
    pub pretty_id: String,
    pub summary: String,
    pub details: Option<String>,
    pub outcome: Option<String>,
    pub assignee: Option<Uuid>,
    pub status: String,
    pub resolved: Option<DateTime<Utc>>,
}

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(req): Json<CreateInvestigationDetails>,
) -> impl IntoResponse {
    let res = Investigation::create(State(state), Extension(user), req).await;

    if res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create investigation".to_string(),
        );
    }

    (StatusCode::OK, json!(res.unwrap()).to_string())
}
