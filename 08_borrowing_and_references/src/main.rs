fn main() {
    let mut sentence = String::from("Hello");
    let s1 = &mut sentence;
    s1.push_str("string");
    let s2 = &mut sentence; // You can have any amount of mutable references of a single variable in a single scope.
    s2.push_str("string");  // You have to make sure that the mutable reference is used before you pass it as another mutable reference to another variable (before going out of scope). Same thing if you need to make a mutatble and immutable reference co-exist.
    update_string(&mut sentence);
    println!("{}", sentence);
}

fn update_string(s: &mut String) {
  s.push_str(" World!");
}
