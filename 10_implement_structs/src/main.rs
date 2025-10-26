struct Rectangle {
  length: u64,
  breadth: u64,
}

impl Rectangle {
  fn area(&self) -> u64 { // Instance methods, accepts &self as the first argument implicitly, represents the address of the object who invoked the method
    return self.length * self.breadth;
  }
  fn permimeter(&self) -> u64 {
    return 2 * (self.length + self.breadth);
  }
  fn new(length: u64, breadth: u64) -> Rectangle { // static method, no &self
    Rectangle { length, breadth }
  }
}

fn main() {
    let rectangle_1 = Rectangle::new(50, 20);
    println!("Area of reactangle is {} and perimeter is {}", rectangle_1.area(), rectangle_1.permimeter());
}
