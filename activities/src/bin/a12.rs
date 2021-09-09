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
    Yellow,
}
struct ShippingBox {
    color: Color,
    dimensions: (f64, f64),
    weight: f64,
}

impl ShippingBox {
    fn new_box(color: Color, dimensions: (f64, f64), weight: f64) -> Self {
        Self {
            color,
            dimensions,
            weight,
        }
    }

    fn display(&self) {
        let color = match self.color {
            Color::Brown => "brown",
            Color::White => "white",
            Color::Yellow => "yellow",
        };
        println!("color: {}", color);
        println!("dimensions: ({}, {})", self.dimensions.0, self.dimensions.1);
        println!("weight: {}", self.weight);
    }
}

fn main() {
    let b = ShippingBox {
        color: Color::Brown,
        dimensions: (12.0, 15.0),
        weight: 38.6,
    };
    b.display();
    let new_box = ShippingBox::new_box(Color::White, (45.0, 50.0), 100.5);
    new_box.display();
}
