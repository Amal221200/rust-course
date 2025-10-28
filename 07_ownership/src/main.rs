fn main() {
    // Ownership occurs in movable types eg:- String, any struct or enum type.
    let name = String::from("Amal Murikkoli");
    let name_1 = name; // Now name is no longer accessible since name_1 is the new owner
    print_name(name_1); // Now name_1 is unaccessible since the data is move into print_name scope.
}

fn print_name(name: String) {
  println!("{}", name)
}