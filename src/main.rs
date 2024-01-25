use axum::{extract::FromRef, Router};
use axum_extra::extract::cookie::Key;
use sqlx::PgPool;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::*;

/// The `api` module takes HTTP requests and forwards them to the underlying functions in `core`,
/// translating the results to HTTP responses and response codes. It also handles reading and
/// writing cookies
mod api;

/// The `core` module is where the "meat" of the application lives. Everything in this module that
/// is triggered by a web request should be called only by request handlers in the `api` module; no
/// `axum` routers or otherwise should exist in `core`
mod core;

#[derive(Clone, FromRef)]
pub struct AppState {
    db: PgPool,
    key: Key,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok().unwrap();

    tracing_subscriber::fmt::init();

    let db = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    // run pending migrations
    sqlx::migrate!().run(&db).await.unwrap();

    // TODO: complain if the key is too short - or if the shannon entropy is too low?
    let key = Key::from(std::env::var("SIGNING_KEY").unwrap().as_bytes());

    let state = AppState { db, key };

    let app = Router::new()
        .nest("/admin", api::admin::router())
        .nest("/auth", api::auth::router())
        // this should be the final router/service to catch all unmatched requests
        // this actually looks for an index.html by default, no fallback router necessary!
        .fallback_service(ServeDir::new("ui/dist"))
        // we validate all of our sessions with each request because the root page needs the user
        // details cookie to be set and whatnot. for a handful of static assets this is unnecessary
        // but for now this additional overhead is likely trivial
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            core::sessions::session_layer,
        ))
        .with_state(state)
        // trace layer is last to catch *everything*
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(std::env::var("LISTEN_ADDRESS").unwrap())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
