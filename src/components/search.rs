use crate::Recipe;

pub fn search_recipe(name: &str, recipes: Vec<Recipe>) -> Option<Recipe> {
    for recipe in &recipes {
        if recipe.name.to_lowercase().contains(&name.to_lowercase()) {
            return Some(recipe.clone()); // Clone the recipe to return an owned value
        }
    }
    None // Return None if no recipe matches
}
