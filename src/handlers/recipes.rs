use crate::handlers::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};

// NOTE: This is a stub implementation to satisfy the generated routing code.
// The Recipe type should match the OpenAPI schema definition when fully implemented.
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub title: String,
}

/// GET /recipes - List all recipes (stub implementation)
pub async fn list_recipes(State(_repo): State<AppState>) -> Json<Vec<Recipe>> {
    // Stub implementation - returns empty list
    Json(vec![])
}

/// POST /recipes - Create a new recipe (stub implementation)
pub async fn create_recipe(
    State(_repo): State<AppState>,
    Json(_payload): Json<Recipe>,
) -> Result<(StatusCode, Json<Recipe>), StatusCode> {
    // Stub implementation - not yet implemented
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// PUT /recipes/{id} - Update a recipe (stub implementation)
pub async fn update_recipe(
    State(_repo): State<AppState>,
    Path(_id): Path<String>,
    Json(_payload): Json<Recipe>,
) -> Result<Json<Recipe>, StatusCode> {
    // Stub implementation - not yet implemented
    Err(StatusCode::NOT_IMPLEMENTED)
}
