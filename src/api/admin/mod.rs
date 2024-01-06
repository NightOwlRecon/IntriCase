use crate::AppState;
use axum::Router;

pub mod users;

pub fn router() -> Router<AppState> {
    Router::new().nest("/users", users::router())
}
