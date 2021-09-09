// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

fn main() {
    let mut furniture = HashMap::new();
    furniture.insert(String::from("chairs"), 5);
    furniture.insert(String::from("beds"), 3);
    furniture.insert(String::from("tables"), 2);
    furniture.insert(String::from("couches"), 0);

    let mut total = 0;
    for (item, amt) in furniture.iter() {
        match amt {
            0 => println!("{} out of stock", item),
            _ => println!("{}: {}", item, amt),
        }
        total += amt;
    }
    println!("{} total items", total);
}
