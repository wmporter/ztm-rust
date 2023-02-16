// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Locker {
    name: String,
    assign: Option<i32>,
}

impl Locker {
    fn display(&self) {
        match self.assign {
            None => println!("{} has no locker", self.name),
            Some(assign) => println!("{} has locker {}", self.name, assign),
        }
    }
}

fn main() {
    let locker = Locker { name: "Bill".to_owned(), assign: None };
    let locker2 = Locker { name: "Gary".to_owned(), assign: Some(46) };
    locker.display();
    locker2.display();
}
