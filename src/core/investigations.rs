use crate::{api::admin::investigations::CreateInvestigationDetails, AppState};
use anyhow::{Error, Result};
use axum::{
    extract::{Request, State},
    Extension,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::{NoContext, Timestamp, Uuid};

use crate::api::admin::investigations::CreateQuestionDetails;
use crate::core::users::User;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, TS)]
#[ts(export)]
pub struct Investigation {
    // core fields
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub creator: Uuid,
    // api fields
    pub internal_id: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub namus_id: Option<String>,
    pub missing_since: NaiveDate,
    pub synopsis: String,
    pub questions: Option<HashMap<Uuid, Question>>,
}

pub async fn get_all(State(state): State<AppState>) -> Result<Vec<Investigation>, sqlx::Error> {
    let res = sqlx::query!("SELECT * FROM investigations ORDER BY last_name ASC")
        .fetch_all(&state.db)
        .await?
        .into_iter()
        // we do this because the questions field does not exist in the database
        // possibly some sqlx trickery possible to make this cleaner, but query_as! does not
        // work with #[sqlx(skip)], for example
        .map(|row| Investigation {
            id: row.id,
            created: row.created,
            creator: row.creator,
            internal_id: row.internal_id,
            first_name: row.first_name,
            middle_name: row.middle_name,
            last_name: row.last_name,
            date_of_birth: row.date_of_birth,
            namus_id: row.namus_id,
            missing_since: row.missing_since,
            synopsis: row.synopsis,
            questions: None,
        })
        .collect::<Vec<Investigation>>();
    Ok(res)
}

impl Investigation {
    pub async fn create(
        State(state): State<AppState>,
        Extension(user): Extension<User>,
        details: CreateInvestigationDetails,
    ) -> Result<Uuid> {
        let inv = sqlx::query!(
            "INSERT INTO investigations (id, created, creator, internal_id, first_name, middle_name, last_name, date_of_birth, namus_id, missing_since, synopsis) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING id;",
            Uuid::new_v7(Timestamp::now(NoContext)),
            Utc::now(),
            user.id,
            details.internal_id,
            details.first_name,
            details.middle_name,
            details.last_name,
            details.date_of_birth,
            details.namus_id,
            details.missing_since,
            details.synopsis,
        ).fetch_one(&state.db).await?;

        for question in details.questions {
            let question =
                Question::create(State(state.clone()), Extension(user.clone()), question).await?;
            for (_id, action_item) in question.action_items {
                ActionItem::create(
                    State(state.clone()),
                    Extension(user.clone()),
                    question.id,
                    action_item.summary,
                    action_item.details,
                )
                .await?;
            }
        }

        Ok(inv.id)
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
            created: inv.created,
            creator: inv.creator,
            internal_id: inv.internal_id,
            first_name: inv.first_name,
            middle_name: inv.middle_name,
            last_name: inv.last_name,
            date_of_birth: inv.date_of_birth,
            namus_id: inv.namus_id,
            missing_since: inv.missing_since,
            synopsis: inv.synopsis,
            questions: None,
        };

        if details {
            investigation.get_questions(State(state)).await?;
        }

        Ok(investigation)
    }

    pub async fn get_questions(&mut self, State(state): State<AppState>) -> Result<()> {
        // if we're doing this we almost certainly want the action items as well
        let questions_data = sqlx::query!(
            r#"SELECT
                questions.id            as question_id,
                questions.created       as question_created,
                questions.pretty_id     as question_pretty_id,
                questions.summary       as question_summary,
                questions.details       as question_details,
                questions.investigation as question_investigation,
                questions.outcome       as question_outcome,
                questions.creator       as question_creator,
                questions.status        as question_status,

                -- we manually mark this field as nullable because sqlx doesn't properly infer with the join
                action_items.id         as "action_item_id?",
                action_items.created    as action_item_created,
                action_items.pretty_id  as action_item_pretty_id,
                action_items.summary    as action_item_summary,
                action_items.details    as action_item_details,
                action_items.outcome    as action_item_outcome,
                action_items.assignee   as action_item_assignee,
                action_items.creator    as action_item_creator,
                action_items.question   as action_item_question,
                action_items.status     as action_item_status,
                action_items.assigned   as action_item_assigned,
                action_items.resolved   as action_item_resolved

                FROM questions
                LEFT JOIN action_items ON action_items.question = questions.id
                WHERE investigation = $1"#,
            self.id
        )
        .fetch_all(&state.db)
        .await
        .map_err(Error::from)?;

        // TODO: refactor query above so this HashMap foolishness is unnecessary
        let mut questions: HashMap<Uuid, Question> = HashMap::new();

        for record in questions_data {
            let question = questions.entry(record.question_id).or_insert(Question {
                id: record.question_id,
                created: record.question_created,
                pretty_id: record.question_pretty_id,
                summary: record.question_summary,
                details: record.question_details,
                investigation: record.question_investigation,
                outcome: record.question_outcome,
                creator: record.question_creator,
                status: record.question_status,
                action_items: HashMap::<Uuid, ActionItem>::new(),
            });

            if let Some(action_item_id) = record.action_item_id {
                let action_item = ActionItem {
                    id: action_item_id,
                    created: record.action_item_created,
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
                };
                question.action_items.insert(action_item_id, action_item);
            }
        }

        self.questions = Some(questions);

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, TS)]
#[ts(export)]
pub struct Question {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub creator: Uuid,
    pub pretty_id: String,
    pub summary: String,
    pub details: Option<String>,
    pub investigation: Uuid,
    pub outcome: Option<String>,
    pub status: String,
    pub action_items: HashMap<Uuid, ActionItem>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, TS)]
#[ts(export)]
pub struct ActionItem {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub creator: Uuid,
    pub pretty_id: String,
    pub summary: String,
    pub details: Option<String>,
    pub outcome: Option<String>,
    pub assignee: Option<Uuid>,
    pub question: Uuid,
    pub status: String,
    pub assigned: Option<DateTime<Utc>>,
    pub resolved: Option<DateTime<Utc>>,
}

impl Question {
    pub async fn get_action_items(&self, State(state): State<AppState>) -> Result<Vec<ActionItem>> {
        sqlx::query_as!(
            ActionItem,
            "SELECT * FROM action_items WHERE question = $1",
            self.id
        )
        .fetch_all(&state.db)
        .await
        .map_err(Error::from)
    }

    pub async fn create(
        State(state): State<AppState>,
        Extension(user): Extension<User>,
        details: CreateQuestionDetails,
    ) -> Result<Question> {
        let res = sqlx::query!(
            "INSERT INTO questions (id, created, creator, pretty_id, summary, details, investigation, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;",
            Uuid::new_v7(Timestamp::now(NoContext)),
            Utc::now(),
            user.id,
            details.pretty_id,
            details.summary,
            details.details,
            details.investigation,
            details.status,
        ).fetch_one(&state.db).await?;
        Ok(Question {
            id: res.id,
            creator: res.creator,
            created: res.created,
            pretty_id: res.pretty_id,
            summary: res.summary,
            details: res.details,
            investigation: res.investigation,
            outcome: res.outcome,
            status: res.status,
            // TODO: see if we can make this or similar happen within sqlx so we can use query_as!()
            action_items: HashMap::new(),
        })
    }

    pub async fn add_action_item(&self, State(state): State<AppState>) {}
    pub async fn status(&self, State(state): State<AppState>) {}
}

impl ActionItem {
    pub async fn create(
        State(state): State<AppState>,
        Extension(user): Extension<User>,
        question: Uuid,
        summary: String,
        details: Option<String>,
    ) -> Result<ActionItem> {
        let res = sqlx::query_as!(
            ActionItem,
            "INSERT INTO action_items (id, created, creator, pretty_id, summary, details, question, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;",
            Uuid::new_v7(Timestamp::now(NoContext)),
            Utc::now(),
            user.id,
            "",
            summary,
            details,
            question,
            "open"
        ).fetch_one(&state.db).await?;
        Ok(res)
    }

    pub async fn get_by_user(State(state): State<AppState>, user: Uuid) -> Result<Vec<ActionItem>> {
        sqlx::query_as!(
            ActionItem,
            "SELECT * FROM action_items WHERE assignee = $1",
            user
        )
        .fetch_all(&state.db)
        .await
        .map_err(Error::from)
    }

    pub async fn update(
        &self,
        State(state): State<AppState>,
        assignee: Uuid,
    ) -> Result<ActionItem> {
        sqlx::query_as!(
            ActionItem,
            "UPDATE action_items SET assignee = $1 WHERE id = $2 RETURNING *",
            assignee,
            self.id
        )
        .fetch_one(&state.db)
        .await
        .map_err(Error::from)
    }
}
