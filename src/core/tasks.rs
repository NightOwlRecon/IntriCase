use anyhow::{Error, Result};
use axum::extract::State;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::AppState;

pub struct Question {
    pub id: Uuid,
    pub pretty_id: String,
    pub summary: String,
    pub details: Option<String>,
    pub investigation: Uuid,
    pub outcome: Option<String>,
    pub creator: Uuid,
    pub status: String,
    pub created: DateTime<Utc>,
}

pub struct ActionItem {
    pub id: Uuid,
    pub pretty_id: String,
    pub summary: String,
    pub details: Option<String>,
    pub outcome: Option<String>,
    pub assignee: Option<Uuid>,
    pub creator: Uuid,
    pub question: Uuid,
    pub status: String,
    pub assigned: DateTime<Utc>,
    pub resolved: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

enum QuestionStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

enum ActionItemStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Question {
    pub fn get_by_user(&self, State(state): State<AppState>, user_id: Uuid) -> Vec<Question> {
        vec![]
    }

    pub async fn action_items(&self, State(state): State<AppState>) -> Result<Vec<ActionItem>> {
        Ok(sqlx::query_as!(
            ActionItem,
            "SELECT * FROM action_items WHERE question = $1",
            self.id
        ).fetch_all(&state.db).await.map_err(Error::from)?)
    }
}

impl ActionItem {

}
