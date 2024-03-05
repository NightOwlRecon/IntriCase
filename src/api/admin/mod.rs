use crate::AppState;
use axum::Router;

pub mod investigations;
pub mod users;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/users", users::router())
        .nest("/investigations", investigations::router())
}
