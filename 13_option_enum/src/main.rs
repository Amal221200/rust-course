fn main() {
    let name = String::from("Amal Murikkoli");
    match name.chars().nth(1) {
      Some(char) => println!("Character at index 1 is {}", char),
      None => println!("No character at index 1")
    }
}
