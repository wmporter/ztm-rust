// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use std::io;

struct Bill {
    name: String,
    amount: String,
}

fn bill_prompt(bills: &mut Vec<Bill>) {
    let mut buf: String = String::new();
    let mut name: String = String::new();
    let mut amount: String = String::new();
    
    println!("Enter bill name: ");
    let status = io::stdin().read_line(&mut buf);
    if status.is_ok() {
        name = buf.to_string();
    }
    println!("Enter amount: ");
    io::stdin().read_line(&mut buf);
    let status = io::stdin().read_line(&mut buf);
    if status.is_ok() {
        amount = buf.to_string();
    }
    let new_bill = Bill {
        name: name,
        amount: amount,
    };
    bills.push(new_bill);
}

fn show_bills(bills: &Vec<Bill>) {
    for bill in bills {
        println!("{:?}", bill.name);
        println!("{:?}", bill.amount);
    }
}

fn inp() -> Option<String> {
    let mut buf = String::new();
    while io::stdin().read_line(&mut buf).is_err() {
        println!("Error. Please re-enter your input");
    }
    let input: String = buf.trim().to_owned();
    if &input == "" {
        None
    }
    else {
        Some(input)
    }
}   

fn main() {
    let mut bills: Vec<Bill> = Vec::new();
    let mut response: String = String::from("");

    loop {
        println!("Choices: ");
        println!("1) Enter a bill");
        println!("2) See existing bills");
        println!("Enter choice: ");
        let choice = inp().expect("bad input").as_str();
        match choice {
            "1" => bill_prompt(&mut bills),
            "2" => show_bills(&bills),
            _ => break,
        }
    }
}
