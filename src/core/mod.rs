use crate::AppState;
use axum::Router;

pub mod crypto;
pub mod helpers;
pub mod sessions;
pub mod users;

pub fn router() -> Router<AppState> {
    Router::new()
    //.route("/login", post(login))
    //.route("/logout", get(logout))
}
