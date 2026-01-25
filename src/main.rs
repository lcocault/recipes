mod handlers;
mod models;
mod repository;

use handlers::AppState;
use repository::InMemoryIngredientRepository;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Include the generated routing code
include!(concat!(env!("OUT_DIR"), "/generated_routes.rs"));

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "recipes_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize repository
    let repo: AppState = Arc::new(InMemoryIngredientRepository::new());

    // Build router using generated code
    let app = build_router(repo);

    // Run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
