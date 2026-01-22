mod handlers;
mod models;
mod repository;

use axum::{
    routing::{get, put},
    Router,
};
use handlers::{create_ingredient, list_ingredients, update_ingredient, AppState};
use repository::InMemoryIngredientRepository;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    // Build router
    let app = Router::new()
        .route(
            "/ingredients",
            get(list_ingredients).post(create_ingredient),
        )
        .route("/ingredients/:id", put(update_ingredient))
        .with_state(repo);

    // Run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
