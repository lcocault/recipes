use std::collections::HashMap;
use std::sync::Mutex;

use crate::core::Ingredient;

#[derive(Default)]
pub struct InMemoryIngredientRepo {
    inner: Mutex<HashMap<String, Ingredient>>,
}

impl InMemoryIngredientRepo {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }

    pub fn save(&self, recipe: Ingredient) {
        let mut lock = self.inner.lock().unwrap();
        lock.insert(recipe.name.clone(), recipe);
    }

    pub fn get(&self, name: &str) -> Option<Ingredient> {
        let lock = self.inner.lock().unwrap();
        lock.get(name).cloned()
    }
}
