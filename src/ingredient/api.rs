use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Ingredient {
    name: String,
    default_unit: String,
    purchase_unit: String,
    price: f64,
}

type SharedState = Arc<Mutex<HashMap<String, Ingredient>>>;

#[tokio::main]
async fn main() {
    let _state: SharedState = Arc::new(Mutex::new(HashMap::new()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    println!("Server startup skipped in this build.");
}

#[allow(dead_code)]
#[axum::debug_handler]
async fn add_ingredient(
    state: axum::extract::State<SharedState>,
    Json(payload): Json<Ingredient>,
) -> Json<String> {
    let mut ingredients = state.lock().unwrap();

    if ingredients.contains_key(&payload.name) {
        return Json("Ingredient already exists".to_string());
    }

    ingredients.insert(payload.name.clone(), payload);
    Json("Ingredient added successfully".to_string())
}
