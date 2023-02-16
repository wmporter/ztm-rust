// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color {
    Brown,
    White,
    Green,
    Black,
}

impl Color {
    fn display(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::White => println!("white"),
            Color::Green => println!("green"),
            Color::Black => println!("black"),
        }
    }
}

struct Dimension {
    height: f64,
    width: f64,
    depth: f64,
}

struct ShippingBox {
    dimensions: Dimension,
    weight: f64,
    color: Color,
}

impl Dimension {
    fn display(&self) {
        println!("h/w/d: {} / {} / {}", self.height, self.width, self.depth);
    }
}
impl ShippingBox {
    fn new_box() -> Self {
        Self {
            dimensions: Dimension { height: 20.0, width: 30.0, depth: 40.0 },
            weight: 123.0,
            color: Color::Brown,
        }
    }
    
    fn display(&self) {
        self.dimensions.display();
        println!("weight: {}", self.weight);
        self.color.display();
    }
}
fn main() {
    let ship = ShippingBox::new_box();
    ship.display();
}
