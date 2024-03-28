use crate::AppState;
use anyhow::{Error, Result};
use axum::extract::State;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::{NoContext, Timestamp, Uuid};

use std::collections::HashMap;

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
    pub questions: Option<HashMap<Uuid, Question>>,
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
            investigation.get_questions(State(state)).await?;
        }

        Ok(investigation)
    }

    pub async fn get_questions(&mut self, State(state): State<AppState>) -> Result<()> {
        // if we're doing this we almost certainly need the action items as well
        let questions_data = sqlx::query!(
            r#"SELECT
                questions.id as question_id,
                questions.pretty_id as question_pretty_id,
                questions.summary as question_summary,
                questions.details as question_details,
                questions.investigation as question_investigation,
                questions.outcome as question_outcome,
                questions.creator as question_creator,
                questions.status as question_status,
                questions.created as question_created,

                -- we manually mark this field as nullable because sqlx doesn't properly infer with the join
                action_items.id as "action_item_id?",
                action_items.pretty_id as action_item_pretty_id,
                action_items.summary as action_item_summary,
                action_items.details as action_item_details,
                action_items.outcome as action_item_outcome,
                action_items.assignee as action_item_assignee,
                action_items.creator as action_item_creator,
                action_items.question as action_item_question,
                action_items.status as action_item_status,
                action_items.assigned as action_item_assigned,
                action_items.resolved as action_item_resolved,
                action_items.created as action_item_created

                FROM questions
                LEFT JOIN action_items ON action_items.question = questions.id
                WHERE investigation = $1"#,
            self.id
        )
        .fetch_all(&state.db)
        .await
        .map_err(Error::from)?;

        let mut questions: HashMap<Uuid, Question> = HashMap::new();

        for record in questions_data {
            let question = questions.entry(record.question_id).or_insert(Question {
                id: record.question_id,
                pretty_id: record.question_pretty_id,
                summary: record.question_summary,
                details: record.question_details,
                investigation: record.question_investigation,
                outcome: record.question_outcome,
                creator: record.question_creator,
                status: record.question_status,
                created: record.question_created,
                action_items: HashMap::<Uuid, ActionItem>::new(),
            });

            if let Some(action_item_id) = record.action_item_id {
                let action_item = ActionItem {
                    id: action_item_id,
                    pretty_id: record.action_item_pretty_id,
                    summary: record.action_item_summary,
                    details: record.action_item_details,
                    outcome: record.action_item_outcome,
                    assignee: record.action_item_assignee,
                    creator: record.action_item_creator,
                    question: record.action_item_question,
                    status: record.action_item_status,
                    assigned: record.action_item_assigned,
                    resolved: record.action_item_resolved,
                    created: record.action_item_created,
                };
                question.action_items.insert(action_item_id, action_item);
            }
        }

        self.questions = Some(questions);

        Ok(())
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
    pub action_items: HashMap<Uuid, ActionItem>,
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
