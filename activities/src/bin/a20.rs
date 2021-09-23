// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum States {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl States {
    fn map_str(state_str: &str) -> Option<States> {
        match state_str.trim().to_lowercase().as_str() {
            "off" => Some(States::Off),
            "sleep" => Some(States::Sleep),
            "reboot" => Some(States::Reboot),
            "shutdown" => Some(States::Shutdown),
            "hibernate" => Some(States::Hibernate),
            _ => None,
        }
    }
}

fn match_state(state: States) {
    use States::*;
    match state {
        Off => println!("turning off"),
        Sleep => println!("night night"),
        Reboot => println!("rebooting now"),
        Shutdown => println!("powering down"),
        Hibernate => println!("sleeping all winter"),
    }
}

fn main() {
    let mut buffer: String = String::from("");
    let status = io::stdin().read_line(&mut buffer);
    if status.is_ok() {
        match States::map_str(&buffer) {
            Some(state) => match_state(state),
            None => println!("invalid state!"),
        }
    }
}
