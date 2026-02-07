// Domain types for the `recipes` feature.

/// Lightweight domain identifier used for examples.
pub type RecipeId = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    pub id: RecipeId,
    pub name: String,
    pub ingredients: Vec<String>,
}

impl Recipe {
    pub fn new(id: impl Into<RecipeId>, name: impl Into<String>, ingredients: Vec<String>) -> Self {
        Self { id: id.into(), name: name.into(), ingredients }
    }
}
