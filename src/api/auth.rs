use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar, PrivateCookieJar};
use serde::Deserialize;
use serde_json::json;
use tracing::*;
use uuid::Uuid;
use zxcvbn::zxcvbn;

use crate::{core::users::User, AppState};

#[derive(Deserialize)]
struct UserLoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct UserActivateRequest {
    user_id: String,
    display_name: String,
    password: String,
    confirm_password: String,
    otp: String,
}

#[derive(Deserialize)]
struct UserPasswordPostureRequest {
    id: String,
    display_name: String,
    password: String,
    confirm: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/activate", post(activate))
        .route("/passwordPosture", post(password_posture))
}

async fn activate(
    State(state): State<AppState>,
    Json(req): Json<UserActivateRequest>,
) -> impl IntoResponse {
    if let Ok(mut user) = User::get_by_id(State(state.clone()), &req.user_id.to_string()).await {
        // TODO: handle errors here, and unify user profile update function
        let _ = user
            .reset(State(state.clone()), &req.otp, &req.password)
            .await;
        let _ = user
            .set_display_name(State(state.clone()), &req.display_name)
            .await;
        return StatusCode::OK;
    }
    StatusCode::UNAUTHORIZED
}

async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    private_jar: PrivateCookieJar,
    Json(body): Json<UserLoginRequest>,
) -> impl IntoResponse {
    // TODO: a login seems to be looking up a user both by email and by id - minimal overhead but ensure necessary
    if let Ok(user) = User::get_by_email(State(state.clone()), &body.email).await {
        if let Ok(session) = user.log_in(State(state), &body.password).await {
            // this should overwrite an existing cookie
            return (
                private_jar.add(Cookie::build(("session", session.id.to_string())).path("/")),
                jar.add(
                    Cookie::build(("user_details", json!(user).to_string()))
                        .path("/")
                        .secure(false)
                        .http_only(false),
                ),
                StatusCode::OK,
            );
        }
    }
    // go ahead and clear the cookies to be safe after a failed login attempt
    (
        private_jar.remove(Cookie::from("session")),
        jar.remove(Cookie::from("user_details")),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
    private_jar: PrivateCookieJar,
) -> impl IntoResponse {
    if let Some(session_id) = private_jar.get("session") {
        if let Ok(session_uuid) = Uuid::parse_str(session_id.value()) {
            let _ = sqlx::query!("DELETE FROM SESSIONS WHERE id = $1", session_uuid)
                .execute(&state.db)
                .await;
        }
    }
    (
        // TODO: we might want to avoid using jar *and* private_jar here if we're just nuking things
        jar.remove(Cookie::build("user_details").path("/")),
        private_jar.remove(Cookie::build("session").path("/")),
        Redirect::temporary("/").into_response(),
    )
}

// TODO: move this to core, to be safe, call it one final time before updating the database
async fn password_posture(
    State(state): State<AppState>,
    Json(req): Json<UserPasswordPostureRequest>,
) -> impl IntoResponse {
    if req.password != req.confirm {
        return json!({"valid": false, "reason": "Passwords do not match"}).to_string();
    }

    // per NIST SP800-63B
    if req.password.len() < 8 {
        return json!({"valid": false, "reason": "Password must be at least eight characters long"}).to_string();
    }

    // limit required to prevent computational DoS, this should be reasonable
    if req.password.len() > 512 {
        return json!({"valid": false, "reason": "Password must be 512 or fewer characters"})
            .to_string();
    }

    if let Ok(user) = User::get_by_id(State(state), &req.id).await {
        if let Ok(entropy) = zxcvbn(&req.password, &[&user.email, &req.display_name]) {
            // per zxcvbn docs, 3 is the minimum threshold for a "good" password
            // will increase this later after playing with some test values
            debug!("ENTROPY {:?}", entropy);
            if entropy.score() > 3 {
                return json!({"valid": true, "reason": ""}).to_string();
            }
        }
        return json!({"valid": false, "reason": "Password complexity too low, or contains parts of user email or display name"}).to_string();
    }
    json!({"valid": false, "reason": "Other error"}).to_string()
}
