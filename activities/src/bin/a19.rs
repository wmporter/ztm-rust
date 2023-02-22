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
    let items: HashMap<&str, i32> = HashMap::from([
        ("Chairs", 5),
        ("Beds", 3),
        ("Tables", 2),
        ("Couches", 0),
    ]);
    
    let mut total = 0;
    for (item, count) in items.iter() {
        match count {
            0 => println!("{} are out of stock", item),
            qty => println!("{} {} in stock", qty, item),
        };
        total += count;
    }
    println!("{total:?} items in stock");
}
