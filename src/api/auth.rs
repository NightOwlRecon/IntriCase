use crate::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

pub fn router() -> Router<AppState> {
    Router::new()
    //.route("/login", post(login))
    //.route("/logout", get(logout))
}

fn log_in(State(state): State<AppState>) {}

fn log_out(State(state): State<AppState>) {}
