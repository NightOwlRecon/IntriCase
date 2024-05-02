use axum::{extract::State, Router};

use crate::core;
use crate::AppState;

pub mod admin;
pub mod auth;
pub mod investigations;
pub mod users;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/admin", admin::router())
        .nest("/investigations", investigations::router())
        .nest("/users", users::router())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            core::sessions::session_layer,
        ))
        //
        // ROUTES BELOW THIS LINE ARE UNAUTHENTICATED
        //
        .nest("/auth", auth::router())
}
