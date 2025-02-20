use crate::Recipe;

use std::io;
pub fn edit_recipe(recipes: &mut Vec<Recipe>, recipe: &mut Recipe)
{
    let mut tmp = Recipe::default();

    tmp.id = recipe.id;
    let mut index = 0;

    for tmp_recipe in &mut *recipes {
        if tmp_recipe.id == recipe.id {
            break;
        }
        index += 1;
    }

    println!("Enter new recipe name or 0 if you don't want to change the name:");
    io::stdin().read_line(&mut tmp.name).expect("Error reading");

    if tmp.name.eq("0") {
        tmp.name = recipe.name.clone();
    }

    println!("Enter recipe description or 0 if you don't want to change the description:");
    io::stdin().read_line(&mut tmp.description).expect("Error reading");

    if tmp.description.eq("0") {
        tmp.description = recipe.description.clone();
    }

    println!("Enter new recipe ingredients or 0 if you don't want to change the ingredients");

    let mut input_tmp = String::new(); 
    io::stdin().read_line(&mut input_tmp).expect("Error reading");

    if input_tmp.eq("0") {
        tmp.ingredients = recipe.ingredients.clone();
    } else { 
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
    }
    
    println!("Enter new recipe instructions or 0 if you don't want to change the recipe instructions");
    
    let mut inp_tmp = String::new(); 
    io::stdin().read_line(&mut inp_tmp).expect("Error reading");

    if inp_tmp.eq("0") {
        tmp.instructions = recipe.instructions.clone();
    } else { 
        loop {
            let mut paragr = String::new();
            println!("Enter recipe instructions paragraph or -1 if you are done:");
            io::stdin().read_line(&mut paragr).expect("Error reading");
    
            if paragr.eq("-1") {
                break;
            }
            tmp.instructions.push(paragr);
        };
    }
    recipes[index] = tmp;
}