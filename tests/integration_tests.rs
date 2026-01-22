use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt;

// Import from the main crate
use recipes_api::{
    handlers::{create_ingredient, list_ingredients, update_ingredient},
    repository::InMemoryIngredientRepository,
};

fn app() -> Router {
    let repo = Arc::new(InMemoryIngredientRepository::new());
    Router::new()
        .route(
            "/ingredients",
            axum::routing::get(list_ingredients).post(create_ingredient),
        )
        .route("/ingredients/:id", axum::routing::put(update_ingredient))
        .with_state(Arc::clone(&repo) as Arc<dyn recipes_api::repository::IngredientRepository>)
}

async fn body_to_json(body: Body) -> Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn test_list_ingredients_empty() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/ingredients")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = body_to_json(response.into_body()).await;
    assert!(body.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_create_ingredient() {
    let app = app();

    let ingredient = json!({
        "name": "Flour",
        "defaultUnit": "kg",
        "purchaseUnit": "kg",
        "price": 2.5,
        "currency": "EUR",
        "density": 0.8,
        "notes": "All-purpose flour"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/ingredients")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&ingredient).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = body_to_json(response.into_body()).await;
    assert_eq!(body["name"], "Flour");
    assert_eq!(body["price"], 2.5);
    assert!(body["id"].is_string());
}

#[tokio::test]
async fn test_create_ingredient_validation_empty_name() {
    let app = app();

    let ingredient = json!({
        "name": "",
        "defaultUnit": "kg",
        "purchaseUnit": "kg",
        "price": 2.5,
        "currency": "EUR"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/ingredients")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&ingredient).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_ingredient_validation_negative_price() {
    let app = app();

    let ingredient = json!({
        "name": "Sugar",
        "defaultUnit": "kg",
        "purchaseUnit": "kg",
        "price": -1.0,
        "currency": "EUR"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/ingredients")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&ingredient).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_list_ingredients_after_create() {
    let app = app();

    // Create first ingredient
    let ingredient1 = json!({
        "name": "Flour",
        "defaultUnit": "kg",
        "purchaseUnit": "kg",
        "price": 2.5,
        "currency": "EUR"
    });

    let app_clone = app.clone();
    app_clone
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/ingredients")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&ingredient1).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Create second ingredient
    let ingredient2 = json!({
        "name": "Sugar",
        "defaultUnit": "kg",
        "purchaseUnit": "kg",
        "price": 1.8,
        "currency": "EUR"
    });

    let app_clone = app.clone();
    app_clone
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/ingredients")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&ingredient2).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // List all ingredients
    let response = app
        .oneshot(
            Request::builder()
                .uri("/ingredients")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = body_to_json(response.into_body()).await;
    let ingredients = body.as_array().unwrap();
    assert_eq!(ingredients.len(), 2);
}

#[tokio::test]
async fn test_update_ingredient() {
    let app = app();

    // Create an ingredient
    let ingredient = json!({
        "name": "Flour",
        "defaultUnit": "kg",
        "purchaseUnit": "kg",
        "price": 2.5,
        "currency": "EUR"
    });

    let app_clone = app.clone();
    let create_response = app_clone
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/ingredients")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&ingredient).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let created_body = body_to_json(create_response.into_body()).await;
    let id = created_body["id"].as_str().unwrap();

    // Update the ingredient
    let updated_ingredient = json!({
        "name": "Whole Wheat Flour",
        "defaultUnit": "kg",
        "purchaseUnit": "kg",
        "price": 3.0,
        "currency": "EUR"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/ingredients/{}", id))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&updated_ingredient).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = body_to_json(response.into_body()).await;
    assert_eq!(body["name"], "Whole Wheat Flour");
    assert_eq!(body["price"], 3.0);
    assert_eq!(body["id"], id);
}

#[tokio::test]
async fn test_update_nonexistent_ingredient() {
    let app = app();

    let ingredient = json!({
        "name": "Flour",
        "defaultUnit": "kg",
        "purchaseUnit": "kg",
        "price": 2.5,
        "currency": "EUR"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/ingredients/nonexistent-id")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&ingredient).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
