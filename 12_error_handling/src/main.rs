use std::fs::read_to_string;

fn main() {
    match read_to_string("./data.txt") {
        // Returns Result<T, E>, a built-in enum that is returned from a function that can throw error or return the data.
        Ok(data) => println!("{}", data), // Ok contains the data
        Err(err) => println!("Error: {}", err), // Err contains the error data that is returned if there was an error.
    }

    // This is how you create a result enum
    // let mut r: Result<String, String> = Result::Ok("()".to_string());
    // r = Result::Err(String::from("Error"));
}
