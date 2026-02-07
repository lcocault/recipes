use crate::scaffold::recipes::core::types::Recipe;
use crate::scaffold::recipes::core::errors::DomainError;

/// Port: repository abstraction for recipes persistence.
pub trait RecipeRepository: Send + Sync {
    fn save(&self, recipe: &Recipe) -> Result<(), DomainError>;
    fn get(&self, id: &str) -> Result<Option<Recipe>, DomainError>;
}
