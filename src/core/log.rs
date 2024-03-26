use anyhow::Result;
use axum::extract::State;
use chrono::{DateTime, Utc};
use uuid::{NoContext, Timestamp, Uuid};

use crate::AppState;

pub struct LogEntry {
    pub id: Uuid,
    pub actor: Option<Uuid>,
    pub target: Option<Uuid>,
    pub previous_data: Option<String>,
    pub data: Option<String>,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl LogEntry {
    pub async fn create(
        State(state): State<AppState>,
        actor: Option<Uuid>,
        target: Option<Uuid>,
        previous_data: Option<String>,
        data: Option<String>,
        message: String,
    ) -> Result<LogEntry> {
        Ok(sqlx::query_as!(
            LogEntry,
            "INSERT INTO logs (id, actor, target, previous_data, data, message, timestamp) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;",
            Uuid::new_v7(Timestamp::now(NoContext)),
            actor,
            target,
            previous_data,
            data,
            message,
            Utc::now(),
        ).fetch_one(&state.db).await?)
    }
}
