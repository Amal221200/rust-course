use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn summarize(&self) -> String {
        // Default implementation, can be overridden
        return format!(
            "The area of the shape is {}sq.m, and perimeter is {}m",
            self.area(),
            self.perimeter()
        );
    }
}

struct Rectangle {
    length: f64,
    breadth: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.breadth
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.breadth)
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

impl Circle {
    fn get_diameter(&self) -> f64 {
        2.0 * self.radius
    }
}

fn get_summary_of_shape(shape: &impl Shape) { // Syntactic sugar for trait bound
  println!("{}", shape.summarize())
}

fn get_summary_of_shape_T<T: Shape>(shape: &T) { // Trait bound
  println!("{}", shape.summarize())
}

fn main() {
    println!("Hello, world!");
}
