use std::{collections::HashMap};

fn main() {
    // _print_len_and_each_character(String::from("Hello World"));
    // println!("{}", _palindrome("plain".to_string()));
    // println!("{}", _vowel_counter("Ishaan".to_string()));
    // println!("{:?}", _concatenate_and_compare("Ishaan".to_string(), "Ishaan".to_string()));
    // println!("{:?}", _char_frequency("Ishaan".to_string()));
    // println!("{}", _title_case("hi manny!".to_string()));
    // println!("{:?}", all_non_consecutive(&[1, 2, 3, 4, 6, 7, 8, 15, 16]));
    let pos = 4;
    println!("Fibonaci at {pos} position is {:?}", fibonacii_number(pos));
}

// Integer and float
fn _sum_and_average(a: i32, b: i32, c: i32) -> (i32, f64) {
    let sum = a + b + c;
    let average = sum as f64 / 3.0;
    (sum, average)
}

fn _celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn _even_odd(num: i32) -> bool {
    num % 2 == 0
}

fn _simple_interest(p: f64, r: f64, t: f64) -> f64 {
    (p * r * t) / 100.0
}

fn _area_perimeter_of_rectangle(l: f64, b: f64) -> (f64, f64) {
    (l * b, 2.0 * (l + b))
}

fn _digit_sum(num: i32) -> i32 {
    let mut result = 0;
    let mut num_cp = num.abs(); // handling negative numbers

    while num_cp != 0 {
        let digit = num_cp % 10;
        result += digit;
        num_cp = num_cp / 10;
    }
    result
}

fn _bmi_calculator(height: f64, weight: f64) -> String {
    let bmi = weight / height.powi(2);
    if bmi < 18.5 {
        String::from("Underweight")
    } else if bmi < 25.0 {
        String::from("Normal")
    } else if bmi < 30.0 {
        String::from("Overweight")
    } else {
        String::from("Obese")
    }
}

// String
fn _print_len_and_each_character(string: String) {
    println!("Length of the string is {}", string.len());
    // println!("Length of the string is {}", string.chars().count());

    for char in string.chars() {
        println!("{}", char);
    }
}

fn _palindrome(string: String) -> bool {
    let rev = string.to_lowercase().chars().rev().collect::<String>();

    return string == rev;
}

fn _vowel_counter(string: String) -> i32 {
    let vowels = "aeiouAEIOU";
    let mut count = 0;
    for char in string.chars() {
        if vowels.contains(char) {
            count += 1
        }
    }
    count
}

fn _concatenate_and_compare(string_1: String, string_2: String) -> (bool, String) {
    // (string_1 == string_2, string_1 + " " + string_2.as_str())
    (string_1 == string_2, format!("{} {}", string_1, string_2))
}

fn _word_reversal(string: String) -> String {
    string
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn _char_frequency(string: String) -> HashMap<char, i32> {
    let mut result = HashMap::new();
    for char in string.chars() {
        let value = result.get(&char);
        result.insert(char, value.unwrap_or(&0) + 1);
    }
    result
}

fn _remove_whitespace(string: String) -> String {
    string.replace(" ", "")
}

fn _title_case(string: String) -> String {
    string
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str().to_lowercase()),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn all_non_consecutive(arr: &[i32]) -> Vec<(usize, i32)> {
    // let mut result: Vec<(usize, i32)> = vec![];
    // if arr.len() == 0 {
    //     return result;
    // }
    // for i in 0..(arr.len() - 1) {
    //     let (current, next) = (arr[i], arr[i + 1]);
    //     if next - current != 1 {
    //         result.push((i + 1, next));
    //     }
    // }
    // for (i, pair) in arr.windows(2).enumerate() {
    //     if pair[1] - pair[0] != 1 {
    //         result.push((i + 1, pair[1]));
    //     }
    // }
    // result
    arr.windows(2).enumerate().filter_map(|(i, pair)| {
      if pair[1] - pair[0] != 1 {
        Some((i + 1, pair[1]))
      } else {
        None
      }
    }).collect()
}

fn fibonacii_number(num: u32) -> u32 {
  // let mut series : Vec<u32> = vec![0, 1];
  // if num < 2 {
  //   return series[num as usize];
  // }
  
  // for i in 2..=num {
  //   let (a, b) = (series[(i - 1) as usize], series[(i - 2) as usize]);
  //   series.push(a + b);
  // }
  
  // return series[series.len() - 1];
  let mut first = 0;
  let mut second = 1;
  
  if num == 0 {
    return first;
  }
  
  if num == 1 {
    return second;
  }
  
  for _ in 1..num - 2 {
    let temp = first;
    first = second;
    second = temp + first;
  }
  
  return second;
}
