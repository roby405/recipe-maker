use crate::Recipe;

use std::io;
pub fn add_recipe(recipes: &mut Vec<Recipe>, id: i32)
{
    let mut tmp = Recipe::default();
    tmp.id = id;
    println!("Enter recipe name:");
    io::stdin().read_line(&mut tmp.name).expect("Error reading");

    println!("Enter recipe description:");
    io::stdin().read_line(&mut tmp.description).expect("Error reading");

    println!("Enter recipe ingredients");
    
    loop {
        println!("Enter ingredient name or -1 if you are done:");
        let mut ing_name = String::new(); 
        let mut ing_qnt = String::new();
        io::stdin().read_line(&mut ing_name).expect("Error reading");

        if ing_name.eq("-1") {
            break;
        }
        println!("Enter ingredient quantity:");
        io::stdin().read_line(&mut ing_qnt).expect("Error reading");
        tmp.ingredients.insert(ing_name, ing_qnt);
    };
    
    println!("Enter recipe instructions");
    
    loop {
        let mut paragr = String::new();
        println!("Enter recipe instructions paragraph or -1 if you are done:");
        io::stdin().read_line(&mut paragr).expect("Error reading");

        if paragr.eq("-1") {
            break;
        }
        tmp.instructions.push(paragr);
    };
    recipes.push(tmp);
}