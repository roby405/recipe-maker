use crate::Recipe;

pub fn delete_recipe(recipes: &mut Vec<Recipe>, recipe: Recipe) {
    let mut counter = 0;
    for rec in &mut *recipes {
        if recipe.id == rec.id {
            recipes.remove(counter);
            break;
        }
        counter += 1;
    }
}