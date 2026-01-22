use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Ingredient data model matching the OpenAPI schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    #[serde(default = "generate_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "defaultUnit")]
    pub default_unit: String,
    #[serde(rename = "purchaseUnit")]
    pub purchase_unit: String,
    pub price: f64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

/// Request body for creating/updating an ingredient (without requiring ID)
#[derive(Debug, Deserialize)]
pub struct CreateIngredientRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "defaultUnit")]
    pub default_unit: String,
    #[serde(rename = "purchaseUnit")]
    pub purchase_unit: String,
    pub price: f64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl From<CreateIngredientRequest> for Ingredient {
    fn from(req: CreateIngredientRequest) -> Self {
        Ingredient {
            id: req.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            name: req.name,
            default_unit: req.default_unit,
            purchase_unit: req.purchase_unit,
            price: req.price,
            currency: req.currency,
            density: req.density,
            notes: req.notes,
        }
    }
}
