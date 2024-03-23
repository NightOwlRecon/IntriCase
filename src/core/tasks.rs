use chrono::{DateTime, Utc};
use uuid::Uuid;

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
    pub assignee: Uuid,
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
    pub fn action_items(&self) -> Vec<ActionItem> {
        vec![]
    }
}

impl ActionItem {

}
