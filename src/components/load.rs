use crate::Recipe;
use std::fs::File;
use std::io::{self, Read};
use bincode::deserialize;

pub fn load_from_file() -> io::Result<Vec<Recipe>> {
    let mut file = File::open("saved_recipes")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let decoded: Vec<Recipe> = deserialize(&buffer).unwrap();
    Ok(decoded)
}