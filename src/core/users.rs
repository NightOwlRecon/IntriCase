use anyhow::{Error, Result};
use axum::extract::State;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use ts_rs::TS;
use url::Url;
use uuid::Uuid;

use crate::core::{crypto, helpers};
use crate::AppState;

use super::sessions::Session;

use std::collections::HashMap;

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
// TODO: define an intermediary User type without sensitive fields to be extra-careful
#[derive(Clone, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    display_name: Option<String>,
    enabled: bool,
    created: DateTime<Utc>,
    #[serde(skip_serializing)]
    otp: Option<String>,
    #[serde(skip_serializing)]
    secret: Option<String>,
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

    pub fn validate_otp(&self, submitted_otp: &str) -> bool {
        if let Some(otp) = &self.otp {
            if let Some(otp_date) = self.otp_date {
                // TODO: load settings like this into AppState
                if let Ok(hours) = std::env::var("OTP_DURATION_HOURS") {
                    if let Ok(hours_int) = hours.parse::<i64>() {
                        if otp_date > Utc::now() - Duration::hours(hours_int) {
                            return submitted_otp == otp;
                        }
                    }
                }
            }
            // TODO: clear otp and date here
        }
        false
    }

    pub async fn create(db: &PgPool, email: &str) -> Result<User> {
        let now = Utc::now();
        let mut user = sqlx::query_as!(
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

        let activation_url = Url::parse(&std::env::var("BASE_URL").unwrap())?
            .join("/#!/activateAccount/")?
            .join(&user.id.to_string())?
            .join(&user.otp.clone().unwrap_or("".to_string()))?;

        if helpers::smtp_enabled() {
            helpers::send_email(
                &user.email,
                "templates/email/activate.hbs",
                "Activate your IntriCase account",
                HashMap::from([("activationUrl", activation_url.as_str())]),
            )
            .await?;
        }

        Ok(user)
    }

    pub async fn new_otp(&mut self, db: &PgPool) -> Result<String> {
        let otp = crypto::gen_otp();
        sqlx::query!(
            r#"UPDATE users SET ("otp", "otp_date") = ($1, $2) WHERE "id" = $3"#,
            otp,
            Utc::now(),
            self.id
        )
        .execute(db)
        .await?;
        self.otp = Some(otp.clone());
        Ok(otp)
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

    pub async fn set_display_name(
        &self,
        State(state): State<AppState>,
        display_name: &str,
    ) -> Result<()> {
        sqlx::query!(
            r#"UPDATE users SET display_name = $1 WHERE id = $2"#,
            display_name,
            self.id
        )
        .execute(&state.db)
        .await
        .unwrap();
        Ok(())
    }

    pub async fn set_password(&self, db: &PgPool, password: &str) -> Result<()> {
        sqlx::query!(
            r#"UPDATE users SET secret = $1 WHERE id = $2"#,
            crypto::hash_password(password)?,
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
        if self.validate_otp(challenge_otp) {
            // set our password
            self.set_password(&state.db, new_password).await?;
            // clear the OTP information
            sqlx::query!(
                "UPDATE users SET otp = NULL, otp_date = NULL WHERE id = $1",
                self.id
            )
            .execute(&state.db)
            .await?;
            return Ok(());
        }
        Err(Error::msg(""))
    }
}

pub async fn get_all(db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users ORDER BY display_name ASC")
        .fetch_all(db)
        .await
}

pub async fn get_enabled(db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE enabled = true ORDER BY display_name ASC")
        .fetch_all(db)
        .await
}
