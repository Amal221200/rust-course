// Define an enum called Shape
enum Shape {
    Circle(f64),         // Variant with associated data (radius)
    Square(f64),         // Variant with associated data (side length)
    Rectangle(f64, f64), // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius.powi(2),
        Shape::Rectangle(length, breadth) => length * breadth,
        Shape::Square(side) => side.powi(2),
    }
}
// if let statements and Option type
fn calculate_circle(shape: Shape) -> Option<f64> {
    if let Shape::Circle(radius) = shape {
        return Some(3.14 * radius.powi(2));
    }
    return None;
}
// Default statement in pattern matching
fn calculate_square_or_rectangle(shape: Shape) -> Option<f64> {
    match shape {
        Shape::Rectangle(length, breadth) => Some(length * breadth),
        Shape::Square(side) => Some(side.powi(2)),
        _ => None,
    }
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    println!("{}", calculate_area(rectangle));
}
