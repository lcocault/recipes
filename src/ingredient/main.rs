use recipes::core::Ingredient;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut ingredients: HashMap<String, Ingredient> = HashMap::new();

    loop {
        println!("\nIngredient Management System");
        println!("1. Add Ingredient");
        println!("2. View Ingredients");
        println!("3. Exit");
        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => add_ingredient(&mut ingredients),
            "2" => view_ingredients(&ingredients),
            "3" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn add_ingredient(ingredients: &mut HashMap<String, Ingredient>) {
    let mut name = String::new();
    let mut default_unit = String::new();
    let mut purchase_unit = String::new();
    let mut price = String::new();

    println!("Enter ingredient name: ");
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    println!("Enter default unit (e.g., g, kg, L): ");
    io::stdin().read_line(&mut default_unit).unwrap();
    let default_unit = default_unit.trim().to_string();

    println!("Enter purchase unit (e.g., kg, L): ");
    io::stdin().read_line(&mut purchase_unit).unwrap();
    let purchase_unit = purchase_unit.trim().to_string();

    println!("Enter price: ");
    io::stdin().read_line(&mut price).unwrap();
    let price: f64 = match price.trim().parse() {
        Ok(p) => p,
        Err(_) => {
            println!("Invalid price. Ingredient not added.");
            return;
        }
    };

    let ingredient = Ingredient::new(name.clone(), default_unit, purchase_unit, price);

    ingredients.insert(name.clone(), ingredient);
    println!("Ingredient '{}' added successfully!", name);
}

fn view_ingredients(ingredients: &HashMap<String, Ingredient>) {
    if ingredients.is_empty() {
        println!("No ingredients available.");
        return;
    }

    println!("\nIngredients:");
    for (name, ingredient) in ingredients {
        println!("- {}: {:?}", name, ingredient);
    }
}
