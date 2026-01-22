use crate::models::{CreateIngredientRequest, Ingredient};
use crate::repository::IngredientRepository;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;

/// Shared application state containing the repository
pub type AppState = Arc<dyn IngredientRepository>;

/// Custom error type for API responses
#[derive(Debug)]
pub enum ApiError {
    NotFound,
    BadRequest(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (status, message).into_response()
    }
}

/// GET /ingredients - List all ingredients
pub async fn list_ingredients(State(repo): State<AppState>) -> Json<Vec<Ingredient>> {
    let ingredients = repo.list();
    Json(ingredients)
}

/// POST /ingredients - Create a new ingredient
pub async fn create_ingredient(
    State(repo): State<AppState>,
    Json(payload): Json<CreateIngredientRequest>,
) -> Result<(StatusCode, Json<Ingredient>), ApiError> {
    // Basic validation
    if payload.name.trim().is_empty() {
        return Err(ApiError::BadRequest("Name cannot be empty".to_string()));
    }
    if payload.price < 0.0 {
        return Err(ApiError::BadRequest("Price cannot be negative".to_string()));
    }

    let ingredient: Ingredient = payload.into();
    let created = repo.create(ingredient);
    Ok((StatusCode::CREATED, Json(created)))
}

/// PUT /ingredients/{id} - Update an existing ingredient
pub async fn update_ingredient(
    State(repo): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<CreateIngredientRequest>,
) -> Result<Json<Ingredient>, ApiError> {
    // Basic validation
    if payload.name.trim().is_empty() {
        return Err(ApiError::BadRequest("Name cannot be empty".to_string()));
    }
    if payload.price < 0.0 {
        return Err(ApiError::BadRequest("Price cannot be negative".to_string()));
    }

    let ingredient: Ingredient = payload.into();
    let updated = repo.update(&id, ingredient).ok_or(ApiError::NotFound)?;

    Ok(Json(updated))
}
