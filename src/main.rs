use axum::{extract::FromRef, Router};
use axum_extra::extract::cookie::Key;
use log::warn;
use sqlx::PgPool;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
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

    let env = std::env::var("ENV").unwrap();

    let cors = match env.as_str() {
        "DEV" => {
            warn!("WARNING: CORS header is very permissive in DEV mode");
            CorsLayer::very_permissive()
        }
        _ => CorsLayer::new(),
    };

    let app = Router::new()
        // for now, the api and each submodule have their own nested routers
        // in the future it might make more sense to have all the routes in one place
        // but for now this keeps things contained/encapsulated and keeps the module structure
        // and the api structure similar to one-another
        .nest("/api", api::router(state.clone()))
        .with_state(state)
        // TODO: now that we've nested the API routes under /api we probably don't need to do this
        // as a fallback if there's a more appropriate or performant method
        .fallback_service(ServeDir::new("ui/dist"))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listen_addr = std::env::var("LISTEN_ADDRESS").unwrap();
    info!("Listening on {}", listen_addr);
    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
