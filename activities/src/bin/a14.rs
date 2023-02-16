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

enum Color {
    Red,
    White,
    Blue,
}

impl Color {
    fn display(&self) {
        match self {
            Color::Red => println!("red"),
            Color::White => println!("white"),
            Color::Blue => println!("blue")
        }
    }
}
struct Person {
    age: i32,
    name: String,
    fave: Color,
}

impl Person {
    fn display(&self) {
        println!("name: {}", self.name);
        println!("age: {}", self.age);
        print!("fave: ");
        self.fave.display();
    }
}

fn main() {
    let people = vec![
        Person { age: 10, name: "Bobby".to_owned(), fave: Color::Blue },
        Person { age: 22, name: "Alice".to_owned(), fave: Color::Red },
        Person { age: 46, name: "Pete".to_owned(), fave: Color::White }
    ];
    
    for person in people {
        if person.name == "Alice" {
            person.display();
        }
    }
}
