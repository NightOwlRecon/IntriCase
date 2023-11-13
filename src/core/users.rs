use anyhow::Result;
use axum::extract::State;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tokio::sync::RwLock;
use tracing::*;
use url::Url;
use uuid::Uuid;

use std::collections::HashMap;
use std::sync::Arc;

use crate::core::{crypto, helpers};
use crate::AppState;

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

    pub async fn set_password(&self, db: &PgPool, password: &str) -> Result<()> {
        let hash = crypto::hash_password(password).unwrap();
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

    pub async fn reset(db: &PgPool, challenge_otp: &str, new_password: &str) {
        let user = sqlx::query_as!(User, r#"SELECT * FROM users WHERE otp = $1"#, challenge_otp)
            .fetch_one(db)
            .await
            .unwrap();
        if let (Some(otp), Some(date)) = (&user.otp, &user.otp_date) {
            // TODO: validate OTP date
            user.set_password(db, new_password).await.unwrap();
        }
    }
}

pub async fn get_all(db: &PgPool) -> Result<HashMap<Uuid, Arc<RwLock<User>>>> {
    let mut users_map = HashMap::<Uuid, Arc<RwLock<User>>>::new();
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(db)
        .await?;
    for user in users {
        // TODO: I don't like us creating the Arc/RwLock here but nothing else is calling this at
        // the moment so we might as well
        users_map.insert(user.id, Arc::new(RwLock::new(user)));
    }
    Ok(users_map)
}
