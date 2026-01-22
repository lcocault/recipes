use crate::models::Ingredient;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Repository trait for ingredient storage operations
pub trait IngredientRepository: Send + Sync {
    fn list(&self) -> Vec<Ingredient>;
    fn create(&self, ingredient: Ingredient) -> Ingredient;
    fn update(&self, id: &str, ingredient: Ingredient) -> Option<Ingredient>;
    fn get(&self, id: &str) -> Option<Ingredient>;
}

/// In-memory implementation of IngredientRepository
#[derive(Clone)]
pub struct InMemoryIngredientRepository {
    storage: Arc<RwLock<HashMap<String, Ingredient>>>,
}

impl InMemoryIngredientRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryIngredientRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl IngredientRepository for InMemoryIngredientRepository {
    fn list(&self) -> Vec<Ingredient> {
        let storage = self.storage.read().unwrap();
        storage.values().cloned().collect()
    }

    fn create(&self, ingredient: Ingredient) -> Ingredient {
        let mut storage = self.storage.write().unwrap();
        let id = ingredient.id.clone();
        storage.insert(id, ingredient.clone());
        ingredient
    }

    fn update(&self, id: &str, ingredient: Ingredient) -> Option<Ingredient> {
        let mut storage = self.storage.write().unwrap();

        if storage.contains_key(id) {
            let mut updated = ingredient;
            updated.id = id.to_string();
            storage.insert(id.to_string(), updated.clone());
            Some(updated)
        } else {
            None
        }
    }

    fn get(&self, id: &str) -> Option<Ingredient> {
        let storage = self.storage.read().unwrap();
        storage.get(id).cloned()
    }
}
