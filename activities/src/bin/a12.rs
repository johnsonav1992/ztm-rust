// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: (f64, f64, f64),
    weight: f64,
    color: Color,
}
// * Use an enum for the box color
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}
// * Implement functionality on the box struct to create a new box
impl ShippingBox {
    fn new(dimensions: (f64, f64, f64), weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!("Dimensions: {:?}", self.dimensions);
        println!("Weight: {}", self.weight);
        println!("Color: {:?}", self.color);
    }
}
// * Implement functionality on the box struct to print the characteristics


fn main() {
    let shipping_box = ShippingBox::new((10.0, 20.0, 30.0), 50.0, Color::Red);
    shipping_box.print_characteristics();
}
