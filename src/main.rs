use axum::{extract::FromRef, Router};
use axum_extra::extract::cookie::Key;
use sqlx::PgPool;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::*;

/// The `api` module takes HTTP requests and forwards them to the underlying functions in `core`,
/// translating the results to HTTP responses and response codes. It also handles reading and
/// writing cookies, as well as additional server-side form validation
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
        // for now, the api and each sub-module have their own nested routers
        // in the future it might make more sense to have all of the routes in one place
        // but for now this keeps things contained/encapsulated and keeps the module structure
        // and the api structure similar to one-another
        .nest("/api", api::router(state.clone()))
        .with_state(state)
        // TODO: now that we've nested the API routes under /api we probably don't need to do this
        // as a fallback if there's a more appropriate or performant method
        .fallback_service(ServeDir::new("ui/dist"))
        .layer(TraceLayer::new_for_http());

    info!("Listening on {}", std::env::var("LISTEN_ADDRESS").unwrap());
    let listener = tokio::net::TcpListener::bind(std::env::var("LISTEN_ADDRESS").unwrap())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
