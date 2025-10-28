// Level 1
// problem 1

// fn print_name(name: String) -> String {
//   println!("{}", name);
//   name
// }
// fn main() {
//     let name = String::from("Amal Murikkoli");
//     let name = print_name(name);
//     print_name(name);
// }

fn print_str(s: &String) {
    println!("{}", s);
}

fn change_str(s: &mut String) {
    print_str(s); // reborrow immutably
    s.push_str(" updated");
}

fn main () {
  let mut name = "Amal Murikkoli".to_string();
  change_str(&mut name);
  print_str(&name);
}