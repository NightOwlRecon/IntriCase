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
use ts_rs::TS;
use uuid::Uuid;

use crate::core::investigations::ActionItem;
use crate::core::users::User;
use crate::{core::helpers::parse_form_date, core::investigations::Investigation, AppState};
pub fn router() -> Router<AppState> {
    Router::new().route("/create", post(create))
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreateInvestigationDetails {
    pub internal_id: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    // `NaiveDate` and other `chrono` structures are serialized/deserialized as RFC3339
    pub date_of_birth: NaiveDate,
    pub namus_id: Option<String>,
    pub missing_since: NaiveDate,
    pub synopsis: String,
    // we use `default` here to give us an empty Vec if the field is not present
    #[serde(default)]
    pub questions: Vec<CreateQuestionDetails>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreateQuestionDetails {
    pub pretty_id: String,
    pub summary: String,
    pub details: Option<String>,
    pub investigation: Uuid,
    pub outcome: Option<String>,
    pub status: String,
    // in theory every question should have a minimum of one action item
    // but we aren't enforcing that
    #[serde(default)]
    pub action_items: Vec<CreateActionItemDetails>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
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
    if let Ok(res) = Investigation::create(State(state), Extension(user), req).await {
        return axum::Json(res).into_response();
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
