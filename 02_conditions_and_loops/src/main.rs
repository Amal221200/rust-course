fn main() {
    println!("{}", get_first_word(String::from("Hello world!")));
}

fn is_even(number: i32) -> bool {
    if number % 2 == 0 { true } else { false }
}

fn print_for_loop() {
    for i in 0..10 {
        println!("{}", i);
    }
}

fn get_first_word(sentence: String) -> String {
  let mut word = String::new();
  for c in sentence.chars() {
    if c == ' ' { // c.is_whitespace()
      break;
    }
    word.push(c);
  }
  
  return word;
}
