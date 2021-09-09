// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
}
fn use_enum(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        _ => println!("other"),
    };
}
fn main() {
    let c = Color::Green;
    use_enum(c);
}
