use crate::Recipe;

pub fn search_recipe(name: &mut String, recipes: &mut Vec<Recipe>) -> Option<Recipe> {
    for recipe in &mut *recipes {
        if recipe.name.to_lowercase().contains(&name.to_lowercase()) {
            return Some(recipe.clone()); // Clone the recipe to return an owned value
        }
    }
    None // Return None if no recipe matches
}
