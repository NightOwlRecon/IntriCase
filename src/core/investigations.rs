use crate::AppState;
use anyhow::{Error, Result};
use axum::extract::State;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
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
    #[sqlx(skip)]
    pub questions: Option<Vec<Question>>,
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
    ) -> Result<Uuid> {
        let res = sqlx::query!(
            "INSERT INTO investigations (id, internal_id, first_name, middle_name, last_name, date_of_birth, namus_id, missing_since, synopsis, created) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id;",
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
        ).fetch_one(&state.db).await?;
        Ok(res.id)
    }

    pub async fn get(
        State(state): State<AppState>,
        id: &str,
        details: bool,
    ) -> Result<Investigation> {
        let inv = sqlx::query!(
            "SELECT * FROM investigations WHERE id = $1",
            Uuid::parse_str(id)?
        )
        .fetch_one(&state.db)
        .await
        .map_err(Error::from)?;

        let mut investigation = Investigation {
            id: inv.id,
            internal_id: inv.internal_id,
            first_name: inv.first_name,
            middle_name: inv.middle_name,
            last_name: inv.last_name,
            date_of_birth: inv.date_of_birth,
            namus_id: inv.namus_id,
            missing_since: inv.missing_since,
            synopsis: inv.synopsis,
            created: inv.created,
            questions: None,
        };

        if details {
            let questions = sqlx::query!(
                "SELECT * FROM questions WHERE investigation = $1",
                inv.id
            ).fetch_all(&state.db).await.map_err(Error::from)?;

            let action_items = sqlx::query!(
                "SELECT * FROM action_items WHERE question = ANY($1)",
                &questions.iter().map(|q| q.id).collect::<Vec<Uuid>>()
            ).fetch_all(&state.db).await.map_err(Error::from)?;
        }

        Ok(investigation)
    }

    pub async fn get_questions(
        &mut self,
        State(state): State<AppState>,
        id: &str,
    ) -> Result<Vec<Question>> {
        Ok(sqlx::query_as!(
            Question,
            "SELECT * FROM questions WHERE investigation = $1",
            Uuid::parse_str(id)?
        )
        .fetch_all(&state.db)
        .await
        .map_err(Error::from)?)
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
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

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
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
    pub assigned: Option<DateTime<Utc>>,
    pub resolved: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
}

impl Question {
    pub async fn action_items(&self, State(state): State<AppState>) -> Result<Vec<ActionItem>> {
        Ok(sqlx::query_as!(
            ActionItem,
            "SELECT * FROM action_items WHERE question = $1",
            self.id
        )
        .fetch_all(&state.db)
        .await
        .map_err(Error::from)?)
    }

    pub async fn add_action_item(&self, State(state): State<AppState>) {}
    pub async fn status(&self, State(state): State<AppState>) {}
}

impl ActionItem {
    pub async fn get_by_user(State(state): State<AppState>, user: Uuid) -> Result<Vec<ActionItem>> {
        Ok(sqlx::query_as!(
            ActionItem,
            "SELECT * FROM action_items WHERE assignee = $1",
            user
        )
        .fetch_all(&state.db)
        .await
        .map_err(Error::from)?)
    }

    pub async fn assign(
        &self,
        State(state): State<AppState>,
        assignee: Uuid,
    ) -> Result<ActionItem> {
        Ok(sqlx::query_as!(
            ActionItem,
            "UPDATE action_items SET assignee = $1 WHERE id = $2 RETURNING *",
            assignee,
            self.id
        )
        .fetch_one(&state.db)
        .await
        .map_err(Error::from)?)
    }

    pub async fn update_status(
        &mut self,
        State(state): State<AppState>,
        status: String,
    ) -> Result<()> {
        sqlx::query_as!(
            ActionItem,
            "UPDATE action_items SET status = $1 WHERE id = $2 RETURNING *",
            status,
            self.id
        )
        .fetch_one(&state.db)
        .await
        .map_err(Error::from)?;
        self.status = status;
        Ok(())
    }
}
