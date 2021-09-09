// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

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
