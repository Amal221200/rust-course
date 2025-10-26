#[derive(Debug)]
struct User {
  name: String,
  email: String,
  active: bool,
  sign_in_count: u64
}

fn main() {
  let user1 = User {
    active: true,
    name: String::from("Bruce Wayne"),
    email: String::from("bruce@wayne.com"),
    sign_in_count: 1
  };
    println!("User 1 {:#?}", user1);
}
