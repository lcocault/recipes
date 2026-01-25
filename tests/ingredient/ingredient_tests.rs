#[cfg(test)]
mod tests {
    use recipes::ingredient::Ingredient;

    #[test]
    fn test_ingredient_creation() {
        let ingredient =
            Ingredient::new("Sugar".to_string(), "kg".to_string(), "kg".to_string(), 2.5);

        assert_eq!(ingredient.name, "Sugar");
        assert_eq!(ingredient.default_unit, "kg");
        assert_eq!(ingredient.purchase_unit, "kg");
        assert_eq!(ingredient.price, 2.5);
    }
}
