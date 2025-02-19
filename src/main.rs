struct Recipe {
    id: i32,
    name: String,
    description: String,
    ingredients: HashMap<String, String>,
    instructions: Vec<String>,
}

fn main() {
    
    let mut recipe_list : Vec<Recipe>;

    while true {
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
        let mut option: i64 = input.trim().parse().expect("Introdu un număr valid");

        match option {
            1 => 
            2 =>
            3 =>
            4 =>
            5 =>
            6 =>
            7 => std::process::exit(0),
            _ => std::process::exit(-1),
        }

    }

}
