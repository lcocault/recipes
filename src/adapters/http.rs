use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::core::Ingredient;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct IngredientDto {
    name: String,
    default_unit: String,
    purchase_unit: String,
    price: f64,
}

impl From<IngredientDto> for Ingredient {
    fn from(d: IngredientDto) -> Self {
        Ingredient::new(d.name, d.default_unit, d.purchase_unit, d.price)
    }
}

type SharedState = Arc<Mutex<HashMap<String, Ingredient>>>;

// Note: this module provides adapter code (HTTP handlers). The real `main`/composition
// should live in the application root and wire adapters to domain ports.

#[allow(dead_code)]
#[axum::debug_handler]
pub async fn add_ingredient(
    state: State<SharedState>,
    Json(payload): Json<IngredientDto>,
) -> Json<String> {
    let mut ingredients = state.0.lock().unwrap();

    if ingredients.contains_key(&payload.name) {
        return Json("Ingredient already exists".to_string());
    }

    ingredients.insert(payload.name.clone(), payload.clone().into());
    Json("Ingredient added successfully".to_string())
}

pub fn make_state() -> SharedState {
    Arc::new(Mutex::new(HashMap::new()))
}
