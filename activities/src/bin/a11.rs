// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_quantity(gi: &GroceryItem) {
    println!("quantity: {:?}", gi.quantity);
}

fn display_id(gi: &GroceryItem) {
    println!("{}", gi.id);
}

fn main() {
    let gi = GroceryItem {
        quantity: 47,
        id: 111,
    };
    
    display_quantity(&gi);
    display_id(&gi);
}
    
