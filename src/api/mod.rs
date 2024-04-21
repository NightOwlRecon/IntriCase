use axum::{extract::State, Router};

use crate::AppState;
use crate::core;

pub mod admin;
pub mod auth;
pub mod investigations;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/api/admin", admin::router())
        .nest("/api/investigations", investigations::router())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            core::sessions::session_layer,
        ))
        //
        // ROUTES BELOW THIS LINE ARE UNAUTHENTICATED
        //
        .nest("/api/auth", auth::router())
}
