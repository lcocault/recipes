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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::InMemoryIngredientRepository;

    fn create_test_request() -> CreateIngredientRequest {
        CreateIngredientRequest {
            id: None,
            name: "Test Ingredient".to_string(),
            default_unit: "kg".to_string(),
            purchase_unit: "kg".to_string(),
            price: 5.0,
            currency: "EUR".to_string(),
            density: Some(1.0),
            notes: Some("Test".to_string()),
        }
    }

    #[tokio::test]
    async fn test_create_ingredient_success() {
        let repo: AppState = Arc::new(InMemoryIngredientRepository::new());
        let request = create_test_request();

        let result = create_ingredient(State(repo.clone()), Json(request)).await;
        assert!(result.is_ok());

        let (status, json) = result.unwrap();
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(json.name, "Test Ingredient");
    }

    #[tokio::test]
    async fn test_create_ingredient_empty_name() {
        let repo: AppState = Arc::new(InMemoryIngredientRepository::new());
        let mut request = create_test_request();
        request.name = "".to_string();

        let result = create_ingredient(State(repo), Json(request)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_ingredient_negative_price() {
        let repo: AppState = Arc::new(InMemoryIngredientRepository::new());
        let mut request = create_test_request();
        request.price = -1.0;

        let result = create_ingredient(State(repo), Json(request)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_ingredients() {
        let repo: AppState = Arc::new(InMemoryIngredientRepository::new());

        // Create two ingredients
        let request1 = create_test_request();
        let _ = create_ingredient(State(repo.clone()), Json(request1))
            .await
            .unwrap();

        let mut request2 = create_test_request();
        request2.name = "Another Ingredient".to_string();
        let _ = create_ingredient(State(repo.clone()), Json(request2))
            .await
            .unwrap();

        // List them
        let result = list_ingredients(State(repo)).await;
        assert_eq!(result.0.len(), 2);
    }

    #[tokio::test]
    async fn test_update_ingredient_success() {
        let repo: AppState = Arc::new(InMemoryIngredientRepository::new());

        // Create an ingredient
        let request = create_test_request();
        let (_, created) = create_ingredient(State(repo.clone()), Json(request))
            .await
            .unwrap();

        // Update it
        let mut update_request = create_test_request();
        update_request.name = "Updated Name".to_string();
        update_request.price = 10.0;

        let result =
            update_ingredient(State(repo), Path(created.id.clone()), Json(update_request)).await;

        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.price, 10.0);
        assert_eq!(updated.id, created.id);
    }

    #[tokio::test]
    async fn test_update_nonexistent_ingredient() {
        let repo: AppState = Arc::new(InMemoryIngredientRepository::new());
        let request = create_test_request();

        let result = update_ingredient(
            State(repo),
            Path("nonexistent-id".to_string()),
            Json(request),
        )
        .await;

        assert!(result.is_err());
    }
}
