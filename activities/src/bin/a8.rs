// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavor {
    Grape,
    Orange,
    Vanilla,
}
struct Drink {
    flavor: Flavor,
    ounces: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Grape => println!("flavor: grape"),
        Flavor::Orange => println!("flavor: orange"),
        Flavor::Vanilla => println!("flavor: vanilla"),
    }
    println!("ounces: {:?}", drink.ounces);
}
fn main() {
    let drink = Drink {
        flavor: Flavor::Orange,
        ounces: 20.0,
    };
    print_drink(drink);
}
