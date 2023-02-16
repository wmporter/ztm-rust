// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let tickets: Vec<Ticket> = vec![
        Ticket::Backstage(64.64, "Dave".to_owned()),
        Ticket::Vip(25.25, "Tom".to_owned()),
        Ticket::Standard(30.30),
    ];
    
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, name) => println!("backstage ticket ${:?} for {}", price, name),
            Ticket::Vip(price, name) => println!("vip ticket ${:?} for {}", price, name),
            Ticket::Standard(price) => println!("standard ticket ${:?}", price),
        }
    }
}
