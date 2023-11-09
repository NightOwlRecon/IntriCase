use axum::{
    http::HeaderValue,
    routing::get,
    Router,
};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // don't care about the results here in case there isn't a .env file or it's malformed
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let db = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();

    // run pending migrations
    sqlx::migrate!().run(&db).await.unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(CorsLayer::new()
            .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
            .allow_methods(Any));

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}