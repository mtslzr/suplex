mod config;
mod db;
mod entities;
mod graphql;

use axum::Router;
use axum::routing::get;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "suplex_api=debug,tower_http=debug".into()),
        )
        .init();

    let config = config::Config::from_env();
    let db = db::connect(&config.database_url).await;

    let schema = graphql::build_schema(db.clone());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .route(
            "/graphql",
            get(graphql::graphiql).post(graphql::graphql_handler),
        )
        .route("/graphiql", get(graphql::graphiql))
        .with_state(schema)
        .layer(cors);

    let addr = format!("0.0.0.0:{}", config.api_port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "ok"
}
