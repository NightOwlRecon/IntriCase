use anyhow::{Error, Result};
use axum::extract::{Request, State};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tokio::sync::RwLock;
use uuid::Uuid;

use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::warn;

use crate::core::crypto;
use crate::AppState;

use super::sessions::Session;

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash or otp code
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("display_name", &self.display_name)
            .field("enabled", &self.enabled)
            .field("created", &self.created)
            .field("secret", &"[redacted]")
            .field("otp", &"[redacted]")
            .field("auth_date", &self.auth_date)
            .field("otp_date", &self.otp_date)
            .finish()
    }
}

// we also skip serializing potentially-sensitive fields here to prevent accidental
// exposure - including fields we wouldn't worry about showing up in debug logs
#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    email: String,
    display_name: Option<String>,
    enabled: bool,
    created: DateTime<Utc>,
    #[serde(skip_serializing)]
    secret: Option<String>,
    #[serde(skip_serializing)]
    otp: Option<String>,
    #[serde(skip_serializing)]
    otp_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    auth_date: Option<DateTime<Utc>>,
}

impl User {
    pub async fn get_by_email(State(state): State<AppState>, email: &str) -> Result<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(&state.db)
            .await?;
        Ok(user)
    }

    pub async fn get_by_id(State(state): State<AppState>, id: &str) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            Uuid::parse_str(id)?
        )
        .fetch_one(&state.db)
        .await?;
        Ok(user)
    }

    pub fn is_active(&self) -> bool {
        self.enabled
    }

    pub fn otp_is_valid(&self) -> bool {
        if let Some(otp_date) = self.otp_date {
            // TODO: load settings like this into AppState
            if let Ok(hours) = std::env::var("OTP_DURATION_HOURS") {
                if let Ok(hours_int) = hours.parse::<i64>() {
                    return otp_date > Utc::now() - Duration::hours(hours_int);
                }
            }
        }
        false
    }

    pub async fn create(db: &PgPool, email: &str) -> Result<User> {
        let now = Utc::now();
        let user = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users 
            ("id", "email", "enabled", "created") 
        VALUES 
            ($1, $2, $3, $4)
        RETURNING *
        "#,
            Uuid::now_v7(),
            email,
            true,
            now,
        )
        .fetch_one(db)
        .await?;

        user.new_otp(db).await?;

        /*helpers::send_email(
            &user.email,
            "templates/email/activate.hbs",
            "Activate your IntriCase account",
            "crads",
        )
        .await?;*/

        Ok(user)
    }

    pub async fn new_otp(&self, db: &PgPool) -> Result<()> {
        let otp = crypto::gen_otp();
        sqlx::query!(
            r#"UPDATE users SET ("otp", "otp_date") = ($1, $2) WHERE "id" = $3"#,
            otp,
            Utc::now(),
            self.id
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn log_in(&self, State(state): State<AppState>, password: &str) -> Result<Session> {
        // validate password
        if self.enabled && self.validate_password(password) {
            return self.create_session(State(state)).await;
        }
        Err(Error::msg("Invalid password or disabled account"))
    }

    fn validate_password(&self, password: &str) -> bool {
        if let Some(hash) = &self.secret {
            // we have a value in the secret field
            if let Ok(is_valid) = crypto::validate_hash(hash, password) {
                // the hash parsed properly
                // return whether the plaintext is valid for the given hash or not
                return is_valid;
            }
        }
        false
    }

    async fn create_session(&self, State(state): State<AppState>) -> Result<Session> {
        Session::create(State(state), self.clone()).await
    }

    pub async fn set_password(&self, db: &PgPool, password: &str) -> Result<()> {
        let hash = crypto::hash_password(password)?;
        sqlx::query!(
            r#"UPDATE users SET secret = $1 WHERE id = $2"#,
            hash,
            self.id
        )
        .execute(db)
        .await
        .unwrap();
        Ok(())
    }

    pub async fn reset(
        &mut self,
        State(state): State<AppState>,
        challenge_otp: &str,
        new_password: &str,
    ) -> Result<()> {
        if let (Some(otp), Some(otp_date)) = (&self.otp, &self.otp_date) {
            // TODO: validate OTP date
            if otp == challenge_otp {
                //&& otp_date > &Utc::now() {
                return self.set_password(&state.db, new_password).await;
            }
        }
        Err(Error::msg(""))
    }
}

pub async fn get_all(db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(db)
        .await
}
