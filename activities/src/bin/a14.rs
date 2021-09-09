// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String,
}

impl Person {
    fn print_name(&self) {
        println!("{}", self.name);
    }

    fn print_color(&self) {
        println!("{}", self.color);
    }
}

fn main() {
    let people = vec![
        Person { age: 3, name: "Hank".to_owned(), color: "brown".to_owned() },
        Person { age: 24, name: String::from("Jill"), color: String::from("red") },
        Person { age: 7, name: "Jeff".to_owned(), color: "gray".to_owned() },
    ];
    for person in people {
        if person.age <= 10 {
            person.print_name();
            person.print_color();
        }
    }
}
