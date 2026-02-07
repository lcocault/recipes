#[cfg(test)]
mod tests {
    use super::types::Recipe;

    #[test]
    fn recipe_new_and_basic_assertions() {
        let r = Recipe::new("r1", "Pancakes", vec!["flour".into(), "eggs".into()]);
        assert_eq!(r.name, "Pancakes");
        assert!(r.ingredients.contains(&"flour".to_string()));
    }
}
