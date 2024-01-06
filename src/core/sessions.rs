use anyhow::Result;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::cookie::{Cookie, Key, PrivateCookieJar};
use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::{NoContext, Timestamp, Uuid};

use std::collections::HashMap;

use crate::{core::users::User, AppState};

pub struct Session {
    id: Uuid,
    user: Uuid,
    created: DateTime<Utc>,
}

fn get_oldest_session_age() -> Result<DateTime<Utc>> {
    let days = std::env::var("SESSION_DURATION_DAYS")?;
    let duration = Duration::days(days.parse::<i64>()?);
    let now = Utc::now();
    let oldest = now - duration;
    Ok(oldest)
}

fn set_session_cookie(State(_state): State<AppState>, _jar: PrivateCookieJar) {}

fn delete_session_cookie(State(_state): State<AppState>, _jar: PrivateCookieJar) {}

pub async fn create_session(State(state): State<AppState>, user: User) -> Result<Session> {
    let session = sqlx::query_as!(
        Session,
        r#"INSERT INTO sessions ("id", "user", "created") VALUES ($1, $2, $3) RETURNING *;"#,
        Uuid::new_v7(Timestamp::now(NoContext)),
        user.id,
        Utc::now(),
    )
    .fetch_one(&state.db)
    .await?;
    Ok(session)
}

pub async fn validate_session(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
) -> Result<bool> {
    if let Some(session_id) = jar.get("session") {
        let uuid = Uuid::try_parse(session_id.value())?;
        /*if let Some(session) = state.sessions.read().unwrap().get(&uuid) {
            let is_valid_age = session.created >= get_oldest_session_age()?;
            return Ok(is_valid_age);
        }*/
    }
    Ok(false)
}

pub async fn get_sessions(db: &PgPool) -> Result<HashMap<Uuid, Session>> {
    let sessions = sqlx::query_as!(Session, "SELECT * FROM sessions")
        .fetch_all(db)
        .await?;
    let mut hashmap_sessions = HashMap::<Uuid, Session>::new();
    for session in sessions {
        hashmap_sessions.insert(session.id.clone(), session);
    }
    Ok(hashmap_sessions)
}

impl Session {}

async fn my_middleware(
    State(state): State<AppState>,
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request,
    next: Next,
) -> Response {
    // do something with `request`...

    let response = next.run(request).await;

    // do something with `response`...

    response
}
