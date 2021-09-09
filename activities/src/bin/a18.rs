// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
struct Customer {
    age: i32,
    name: String,
}

fn can_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        return Ok(())
    }
    Err(String::from("Under 21"))
}
fn main() {
    let kid = Customer {
        age: 12,
        name: String::from("Jeff")
    };
    match can_purchase(&kid) {
        Ok(_) => println!("{} can buy", kid.name),
        Err(msg) => println!("{} cannot buy: {}", kid.name, msg),
    }
}
