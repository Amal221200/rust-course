fn main() {
    let longest_str;
    let str_1 = String::from("Amal");
    let str_2 = String::from("Shinoj");
    longest_str = longest(&str_1, &str_2);
    // {
    //   let str_2 = String::from("Shinoj");
    //   longest_str = longest(&str_1, &str_2);
    //   println!("{}", longest_str);
    // }
    println!("{}", longest_str);

    let name = String::from("Amal");
    // Lifetimes with struct
    let person = Person {
        name: &name,
        age: 24,
    };
}

// Lifetimes are a way to tell the compiler that the reference a function returns will live at least as long as the references of arguments are valid — in other words, it’s valid for the shortest overlap of their lifetimes.

// Lifetime make sure that the references that are taking input from is living long enough (is still available in memory) for the function to execute.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

struct Person<'a> {
    name: &'a str,
    age: u32,
}
