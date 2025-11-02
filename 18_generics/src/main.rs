fn main() {
    println!("{}", find_largest('a', 'z'));
}

// Without generics
// fn find_largest_number (a: i32, b: i32) -> i32 {
//  if (a > b) {
//    a
//  } else {
//    b
//  }
// }
// 
// fn find_largest_string (a: char, b: char) -> char {
//  if (a > b) {
//    a
//  } else {
//    b
//  }
// }

// With generics
fn find_largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
  if a > b {
    a
  } else {
    b
  }
}