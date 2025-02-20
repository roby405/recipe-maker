use crate::Recipe;

use std::io;
pub fn edit_recipe(recipes: &mut Vec<Recipe>, recipe: &Recipe)
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
    tmp.name.pop();
    if tmp.name.eq("0") {
        tmp.name = recipe.name.clone();
    }

    println!("Enter recipe description or 0 if you don't want to change the description:");
    io::stdin().read_line(&mut tmp.description).expect("Error reading");
    tmp.description.pop();

    if tmp.description.eq("0") {
        tmp.description = recipe.description.clone();
    }

    println!("Enter 0 if you don't want to change the ingredients or anything else if you do");

    let mut input_tmp = String::new(); 
    io::stdin().read_line(&mut input_tmp).expect("Error reading");
    input_tmp.pop();

    if input_tmp.eq("0") {
        tmp.ingredients = recipe.ingredients.clone();
    } else { 
        loop {
            println!("Enter ingredient name or -1 if you are done:");
            let mut ing_name = String::new(); 
            let mut ing_qnt = String::new();
            io::stdin().read_line(&mut ing_name).expect("Error reading");
            ing_name.pop();

            if ing_name.eq("-1") {
                break;
            }
            println!("Enter ingredient quantity:");
            io::stdin().read_line(&mut ing_qnt).expect("Error reading");
            tmp.ingredients.insert(ing_name, ing_qnt);
        };
    }
    
    println!("Enter 0 if you don't want to change the recipe instructions or anything else if you do");
    
    let mut inp_tmp = String::new(); 
    io::stdin().read_line(&mut inp_tmp).expect("Error reading");
    inp_tmp.pop();

    if inp_tmp.eq("0") {
        tmp.instructions = recipe.instructions.clone();
    } else { 
        loop {
            let mut paragr = String::new();
            println!("Enter recipe instructions paragraph or -1 if you are done:");
            io::stdin().read_line(&mut paragr).expect("Error reading");
            paragr.pop();
    
            if paragr.eq("-1") {
                break;
            }
            tmp.instructions.push(paragr);
        };
    }
    recipes[index] = tmp;
    println!("Recipe was modified succesfully!");
}
