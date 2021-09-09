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
    assignment: Option<i32>,
}

fn main() {
    let william = Locker {
        name: String::from("William"),
        assignment: Some(490),
    };
    let jenny = Locker {
        name: String::from("Jenny"),
        assignment: None,
    };

    println!("student {:?} locker {:?}", william.name, william.assignment);
    println!("student {:?} locker {:?}", jenny.name, jenny.assignment);
}
