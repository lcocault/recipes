use crate::models::Ingredient;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Repository trait for ingredient storage operations
#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_ingredient(name: &str) -> Ingredient {
        Ingredient {
            id: "test-id".to_string(),
            name: name.to_string(),
            default_unit: "kg".to_string(),
            purchase_unit: "kg".to_string(),
            price: 2.5,
            currency: "EUR".to_string(),
            density: Some(1.0),
            notes: Some("Test notes".to_string()),
        }
    }

    #[test]
    fn test_create_ingredient() {
        let repo = InMemoryIngredientRepository::new();
        let ingredient = create_test_ingredient("Flour");

        let created = repo.create(ingredient.clone());
        assert_eq!(created.name, "Flour");
        assert_eq!(created.id, ingredient.id);
    }

    #[test]
    fn test_list_ingredients() {
        let repo = InMemoryIngredientRepository::new();
        let ingredient1 = create_test_ingredient("Flour");
        let mut ingredient2 = create_test_ingredient("Sugar");
        ingredient2.id = "test-id-2".to_string();

        repo.create(ingredient1);
        repo.create(ingredient2);

        let list = repo.list();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_update_ingredient() {
        let repo = InMemoryIngredientRepository::new();
        let ingredient = create_test_ingredient("Flour");
        repo.create(ingredient.clone());

        let updated = create_test_ingredient("Whole Wheat Flour");
        let result = repo.update(&ingredient.id, updated.clone());

        assert!(result.is_some());
        let updated_ingredient = result.unwrap();
        assert_eq!(updated_ingredient.name, "Whole Wheat Flour");
        assert_eq!(updated_ingredient.id, ingredient.id);
    }

    #[test]
    fn test_update_nonexistent_ingredient() {
        let repo = InMemoryIngredientRepository::new();
        let ingredient = create_test_ingredient("Flour");

        let result = repo.update("nonexistent-id", ingredient);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_ingredient() {
        let repo = InMemoryIngredientRepository::new();
        let ingredient = create_test_ingredient("Flour");
        repo.create(ingredient.clone());

        let result = repo.get(&ingredient.id);
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Flour");
    }

    #[test]
    fn test_get_nonexistent_ingredient() {
        let repo = InMemoryIngredientRepository::new();
        let result = repo.get("nonexistent-id");
        assert!(result.is_none());
    }
}
