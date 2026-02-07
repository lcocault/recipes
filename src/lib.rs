pub mod adapters;
pub mod core;

// Re-export generated OpenAPI clients when available (generated at build/CI time)
#[cfg(feature = "with-generated")]
pub mod generated {
    pub use ingredient_api;
    pub use recipe_api;
}
