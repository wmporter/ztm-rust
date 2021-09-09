// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add_two(one: i32, two: i32) -> i32 {
    return one + two;
}

fn display(result: i32) {
    println!("{:?}", result);
}

fn main() {
    let res = add_two(5, 2);
    display(res);
}
