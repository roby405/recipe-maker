use crate::Recipe;

pub fn view_details(recipe: &Recipe) {
    println!("Description: {}", recipe.description);
    println!("Ingredients:");
    for (key, val) in &recipe.ingredients {
        println!("{key} --- {val}");
    }
    let mut counter: i32 = 1;
    for step in &recipe.instructions {
        println!("{counter}. {step}");
        counter+=1;
    }
    println!("");
    println!("I hope you have a nice time!");
}