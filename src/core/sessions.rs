use anyhow::Result;
use axum::response::{IntoResponse, Redirect};
use axum::{
    extract::{Request, State},
    middleware::Next,
};
use axum_extra::extract::{cookie::Cookie, CookieJar, PrivateCookieJar};
use chrono::{DateTime, Duration, Utc};
use uuid::{NoContext, Timestamp, Uuid};

use crate::{core::users::User, AppState};

pub struct Session {
    pub id: Uuid,
    user: Uuid,
    created: DateTime<Utc>,
}

impl Session {
    pub async fn get_by_id(State(state): State<AppState>, id: &str) -> Result<Session> {
        let uuid = Uuid::try_parse(id)?;
        let session = sqlx::query_as!(Session, r#"SELECT * FROM sessions WHERE id = $1"#, uuid)
            .fetch_one(&state.db)
            .await?;
        Ok(session)
    }

    pub async fn create(State(state): State<AppState>, user: User) -> Result<Session> {
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

    pub fn is_valid(&self) -> bool {
        // TODO: load settings like this into AppState
        if let Ok(days) = std::env::var("SESSION_DURATION_DAYS") {
            if let Ok(days_int) = days.parse::<i64>() {
                return self.created > Utc::now() - Duration::days(days_int);
            }
        }
        false
    }

    pub async fn delete(&self, State(state): State<AppState>) -> Result<()> {
        sqlx::query!("DELETE FROM sessions WHERE id = $1", self.id)
            .execute(&state.db)
            .await?;
        Ok(())
    }
}

pub async fn session_layer(
    State(state): State<AppState>,
    jar: CookieJar,
    private_jar: PrivateCookieJar,
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    // TODO: see about simplifying these nested if statements
    if let Some(session_id) = private_jar.get("session") {
        // we have an encrypted session cookie
        if let Ok(session) = Session::get_by_id(State(state.clone()), session_id.value()).await {
            // the cookie corresponds to an existing session id
            if session.is_valid() {
                // the session is valid
                if let Ok(user) =
                    User::get_by_id(State(state.clone()), &session.user.to_string()).await
                {
                    // the user from the session exists
                    if user.is_active() {
                        // the user is active, so we add the user to the request extensions
                        request.extensions_mut().insert(user);
                        return (
                            // TODO: update user data if anything has changed
                            jar,
                            private_jar,
                            next.run(request).await,
                        );
                    }
                }
            } else {
                // session isn't valid, go ahead and nuke it
                session.delete(State(state.clone())).await.unwrap();
            }
        }
    }
    // no session set, or invalid session
    // TODO: deduplicate this with logout?
    (
        jar.remove(Cookie::build("user_details").path("/")),
        private_jar.remove(Cookie::build("session").path("/")),
        Redirect::temporary("/").into_response(),
    )
}
