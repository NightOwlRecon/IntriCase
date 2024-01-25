use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar, SignedCookieJar};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{core::users::User, AppState};

#[derive(Deserialize)]
struct UserLoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct UserActivateRequest {
    display_name: String,
    password: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/activate/:user_id/:otp", get(activate))
}

async fn activate(
    State(state): State<AppState>,
    Path((user_id, otp)): Path<(Uuid, String)>,
    jar: SignedCookieJar,
) -> impl IntoResponse {
    let user = User::get_by_id(State(state.clone()), &user_id.to_string()).await;
    (jar.remove("session"), StatusCode::OK)
}

async fn login(
    State(state): State<AppState>,
    jar: SignedCookieJar,
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
    jar: SignedCookieJar,
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
        jar.remove(Cookie::from("user_details")),
        private_jar.remove(Cookie::from("session")),
        Redirect::temporary("/"),
    )
}
