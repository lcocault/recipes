#[allow(dead_code)]
#[derive(Debug)]
pub struct Ingredient {
    pub name: String,
    pub default_unit: String,
    pub purchase_unit: String,
    pub price: f64,
}

impl Ingredient {
    pub fn new(name: String, default_unit: String, purchase_unit: String, price: f64) -> Self {
        Self {
            name,
            default_unit,
            purchase_unit,
            price,
        }
    }
}
