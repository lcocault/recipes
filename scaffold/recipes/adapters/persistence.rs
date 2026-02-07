// Persistence adapter stub (in-memory) for the `recipes` module.

use crate::scaffold::recipes::core::ports::RecipeRepository;
use crate::scaffold::recipes::core::types::Recipe;
use crate::scaffold::recipes::core::errors::DomainError;

use std::collections::HashMap;
use std::sync::Mutex;

pub struct InMemoryRepo {
    inner: Mutex<HashMap<String, Recipe>>,
}

impl InMemoryRepo {
    pub fn new() -> Self {
        Self { inner: Mutex::new(HashMap::new()) }
    }
}

impl RecipeRepository for InMemoryRepo {
    fn save(&self, recipe: &Recipe) -> Result<(), DomainError> {
        let mut lock = self.inner.lock().map_err(|e| DomainError::Unexpected(e.to_string()))?;
        lock.insert(recipe.id.clone(), recipe.clone());
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Option<Recipe>, DomainError> {
        let lock = self.inner.lock().map_err(|e| DomainError::Unexpected(e.to_string()))?;
        Ok(lock.get(id).cloned())
    }
}
