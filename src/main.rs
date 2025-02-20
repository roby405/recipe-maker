mod components {
    pub mod add;
    pub mod delete;
    pub mod edit;
    pub mod search;
    pub mod view_all;
    pub mod view_details;
    pub mod save;
    pub mod load;
}

use std::collections::HashMap;

use components::{add::add_recipe, search::search_recipe, view_all::view_all, view_details::view_details, delete::delete_recipe, edit::edit_recipe, save::save, load::load_from_file};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
struct Recipe {
    id: i32,
    name: String,
    description: String,
    ingredients: HashMap<String, String>,
    instructions: Vec<String>,
}

fn main() {
    
    let mut recipe_list : Vec<Recipe> = Vec::new();
    let mut id = 0;

    match load_from_file() {
        Ok(list) => {
            recipe_list = list;
        }
        Err(e) => {
            eprintln!("Failed to load recipes: {}", e);
        }
    }

    loop {
        println!("What would you like to do?");
        println!("1.Add recipe");
        println!("2.View recipe list");
        println!("3.View recipe details");
        println!("4.Search recipe");
        println!("5.Edit recipe");
        println!("6.Delete recipe");
        println!("7.Exit!");
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("Nu am putut citi linia");
        let option: i64 = input.trim().parse().expect("Introdu un număr valid");

        match option {
            1 => {
                add_recipe(&mut recipe_list, id);
                id += 1;
                save(&recipe_list).unwrap();
            }
            2 => view_all(&mut recipe_list),
            3 => {
                println!("Enter recipe name");
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).expect("Error reading");
                s.pop();
                let recipe = search_recipe(&mut s, &mut recipe_list);
                match recipe {
                    Some(recipe) => view_details(&recipe),
                    None => std::process::exit(-1),
                }
            }
            4 => {
                println!("Enter recipe name:");
                let mut wanted_recipe = String::new();
                std::io::stdin().read_line(&mut wanted_recipe).expect("Error reading");
                wanted_recipe.pop();
                let recipe = search_recipe(&mut wanted_recipe, &mut recipe_list);
                match recipe {
                    Some(recipe) => println!("Recipe name is : {}", recipe.name),
                    None => {
                        println!("Recipe wasn't found!");
                        std::process::exit(-1)
                    },
                }
            }
            5 => {
                println!("Enter recipe name:");
                let mut wanted_recipe = String::new();
                std::io::stdin().read_line(&mut wanted_recipe).expect("Error reading");
                wanted_recipe.pop();
                let recipe = search_recipe(&mut wanted_recipe, &mut recipe_list);
                match recipe {
                    Some(recipe) => edit_recipe(&mut recipe_list, &recipe),
                    None => std::process::exit(-1),
                }
                save(&recipe_list).unwrap();
            }
            6 => {
                println!("Enter recipe name");
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).expect("Error reading");
                s.pop();
                let recipe = search_recipe(&mut s, &mut recipe_list);
                match recipe {
                    Some(recipe) => delete_recipe(&mut recipe_list, recipe),
                    None => std::process::exit(-1),
                }
            }
            7 => std::process::exit(0),
            _ => std::process::exit(-1),
        }

    }

}
