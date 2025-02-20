use crate::Recipe;

pub fn view_all(recipes: Vec<Recipe>) {
    let mut counter: i32 = 1;
    for recipe in &recipes {
        println!("{counter}. {}", recipe.name);
        counter += 1;
    }
}