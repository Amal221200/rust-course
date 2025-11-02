fn main() {
    let mut name = String::from("Amal Murikkoli");
    name.clear();
    find_first_word(&name);
    let s = name.as_str();
    let first_name = &name[0..5]; // string slice
    let literal = "String"; // string literal also a string slice
    println!("{}", s.split(" ").nth(0).unwrap_or("default"));
    
    // It also works with arrays/vectors (ordered collections)
    let nums = vec![1, 2, 5, 7 ,0];
    let s_1 = &nums[0..4];
    let a = [3, 4];
    let s_2 = &a[0..];
}

fn find_first_word(sentence: &String) -> &str {
  let mut space_index = 0;
  for char in sentence.chars() {
    if char == ' ' {
      break;
    }
    space_index += 1;
  }
  &sentence[0..space_index]
}