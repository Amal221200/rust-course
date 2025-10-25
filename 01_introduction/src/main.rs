fn main() {
    let name = String::from("Amal Murikkoli");
    let pi = 3.14;
    let age = 25;
    let is_student = true;
    println!("Hello, {}!", name);
    println!("Welcome to Rust!");
    println!("Let's learn Rust together!");
    
    match name.chars().nth(8) {
        Some(c) => println!("The first character of your name is: {}", c),
        None => println!("Your name is empty!"),
    }
}
