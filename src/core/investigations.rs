use crate::AppState;
use anyhow::{Error, Result};
use axum::extract::State;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::{NoContext, Timestamp, Uuid};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Investigation {
    pub id: Uuid,
    pub internal_id: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub namus_id: Option<String>,
    pub missing_since: NaiveDate,
    pub synopsis: String,
    pub created: DateTime<Utc>,
}

impl Investigation {
    // TODO: avoid using so many fields if possible
    pub async fn create(
        State(state): State<AppState>,
        internal_id: Option<String>,
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
        date_of_birth: NaiveDate,
        namus_id: Option<String>,
        missing_since: NaiveDate,
        synopsis: String,
    ) -> Result<Investigation> {
        Ok(sqlx::query_as!(
            Investigation,
            "INSERT INTO investigations (id, internal_id, first_name, middle_name, last_name, date_of_birth, namus_id, missing_since, synopsis, created) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *;",
            Uuid::new_v7(Timestamp::now(NoContext)),
            internal_id,
            first_name,
            middle_name,
            last_name,
            date_of_birth,
            namus_id,
            missing_since,
            synopsis,
            Utc::now(),
        ).fetch_one(&state.db).await?)
    }

    pub async fn get(State(state): State<AppState>, id: &str) -> Result<Investigation> {
        sqlx::query_as!(
            Investigation,
            "SELECT * FROM investigations WHERE id = $1",
            Uuid::parse_str(id)?
        )
        .fetch_one(&state.db)
        .await
        .map_err(Error::from)
    }

    pub async fn get_all(db: PgPool) -> Result<Vec<Investigation>> {
        sqlx::query_as!(Investigation, "SELECT * FROM investigations")
            .fetch_all(&db)
            .await
            .map_err(Error::from)
    }
}
