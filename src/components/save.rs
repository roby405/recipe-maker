use std::fs::File;
use std::io::{self, Write};

use crate::Recipe;
use bincode::serialize;


pub fn save(recipes: &Vec<Recipe>) -> io::Result<()> {
    let serialized = serialize(recipes).unwrap();
    let mut file = File::create("saved_recipes")?;
    file.write_all(&serialized)?;
    Ok(())
}