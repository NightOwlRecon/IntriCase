use axum::{extract::FromRef, Router};
use axum_extra::extract::cookie::Key;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
// tokio's RwLock is provides some additional guarantees versus the os-dependent stdlib's
use crate::core::sessions::Session;
use tokio::sync::RwLock;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::*;
use uuid::Uuid;

/// The `api` module takes HTTP requests and forwards them to the underlying functions in `core`,
/// translating the results to HTTP responses and response codes. It also handles reading and
/// writing cookies
mod api;

/// The `core` module is where the "meat" of the application lives. Everything in this module that
/// is triggered by a web request should be called only by request handlers in the `api` module; no
/// `axum` routers or otherwise should exist in `core`
mod core;
use core::users::User;

#[derive(Clone)]
struct AppState {
    db: PgPool,
    key: Key,
    // TODO: for users and sessions, with a large number of users or concurrent sessions this could end up eating a lot
    // of memory - a caching layer such as moka may be helpful here
    users: HashMap<Uuid, Arc<RwLock<User>>>,
    // tower-sessions implements this in a similar manner using Mutexes but I think RwLocks are a better fit
    sessions: HashMap<Uuid, Arc<RwLock<Session>>>,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
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

    info!("Loading users...");
    let users = core::users::get_all(&db).await.unwrap();
    info!("Loaded {} users", users.len());

    info!("Loading sessions...");
    let sessions = HashMap::<Uuid, Arc<RwLock<Session>>>::new();
    info!("Loaded {} sessions", sessions.len());

    let state = AppState {
        db,
        key,
        users,
        sessions,
    };

    // this actually looks for an index.html by default,
    // no fallback router necessary!
    let serve_ui = ServeDir::new("ui/dist");

    let app = Router::new()
        .nest("/admin", api::admin::router())
        .nest("/auth", api::auth::router())
        // establish state for all of our dynamic routes
        .with_state(state)
        // we don't need CORS for static assets so this can be layered after the fallback
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .fallback_service(serve_ui)
        // this should be the last item to catch all requests
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(std::env::var("LISTEN_ADDRESS").unwrap())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
