
fn main() {
  update_string();
}

fn update_string() {
  let mut sentence = String::from("My name is Amal.");
  println!("Length: {}, Capacity: {}, Pointer: {:p}", sentence.len(), sentence.capacity(), &sentence);
  sentence.push_str("I am learning Rust");
  println!("Length: {}, Capacity: {}, Pointer: {:p}", sentence.len(), sentence.capacity(), &sentence);
  sentence.push_str("I am learning Rust");
  println!("Length: {}, Capacity: {}, Pointer: {:p}", sentence.len(), sentence.capacity(), &sentence);
  sentence.push_str("I am learning Rust");
  println!("Length: {}, Capacity: {}, Pointer: {:p}", sentence.len(), sentence.capacity(), &sentence);
}
