use std::ops::Add;

// Level 1
// Problem 1
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Problem 2
struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn to_fahrenheit(&self) -> f64 {
        (self.celsius * (9.0 / 5.0)) + 32.0
    }

    fn from_fahrenheit(farenheit: f64) -> Temperature {
        let celsius = (farenheit - 32.0) * (5.0 / 9.0);
        Temperature { celsius }
    }
}

// Level 2
// Problem 2
struct Account {
  balance: f64,
  owner: String,
}

enum Transaction {
  Deposit(f64),
  Withdraw(f64)
}

impl Account {
    fn apply(&mut self, tx: Transaction) -> Result<(), String> {
      match tx {
         Transaction::Deposit(amount) => {
           if amount < 0.0 {
             Err(String::from("Amount can't be less than zero."))
           } else {
             self.balance += amount;
             Ok(())
           }
         },
         Transaction::Withdraw(amount) => {
           if amount > self.balance {
             Err(String::from("You don't have enough balance"))
           } else {
             self.balance -= amount;
             Ok(())
           }
         },
      }
    }
}

// Level 3
// Problem 1
fn safe_divide(dividend: f64, divisor: f64) -> Result<f64, String> {
  if divisor == 0.0 {
   return Err("Zero division error".to_string());
  }
  Ok(dividend / divisor)
}

// Problem 2
fn file_reader_simulator(file_name: String) -> Result<Option<String>, String> {
  if file_name == String::from("config.txt") {
    Ok(Some(String::from("user==admin")))
  } else if file_name == String::from("empty.txt") {
    Ok(None)
  } else {
    Err(String::from("File not found"))
  }
}

// Level 4
// Problem 1
// #[derive(Clone)]
struct Book { title: String, available: bool }
struct Library { books: Vec<Book> }

enum LibraryError { BookNotFound, AlreadyBorrowed, AlreadyAvailable }

impl Library {
  fn find_book(&self, title: String) -> Option<&Book> {
    match self.books.iter().find(|book| book.title == title) {
      Some (book) => Some(book),
      _ => None
    }
  }
  
  // Unoptimized, but a reminder of my learning
  // fn borrow_book(&mut self, title: String) -> Result<(), LibraryError> {
  //   let book = self.find_book(title.clone());
    
  //   match book {
  //     Some(book) => {
  //       if !book.available {
  //         Err(LibraryError::AlreadyBorrowed)
  //       } else {
  //         self.books = self.books.iter().map(|book| {
  //           if book.title == title {
  //             return Book {
  //               available: false,
  //               ..book.clone()
  //             }
  //           } else {
  //             book.clone()
  //           }
  //         }).collect();
  //         Ok(())
  //       }
  //     },
  //     None => Err(LibraryError::BookNotFound)
  //   }
  // }
  
  // fn return_book(&mut self, title: String) -> Result<(), LibraryError> {
  //   let book = self.find_book(title.clone());
    
  //   match book {
  //     Some(book) => {
  //       if !book.available {
  //         self.books = self.books.iter().map(|book| {
  //           if book.title == title {
  //             return Book {
  //               available: true,
  //               ..book.clone()
  //             }
  //           } else {
  //             book.clone()
  //           }
  //         }).collect();
  //         Ok(())
  //       } else {
  //         Err(LibraryError::AlreadyAvailable)
  //       }
  //     },
  //     None => Err(LibraryError::BookNotFound)
  //   }
  // }
  
  // Optimized, learning from previous mistakes
  fn op_borrow_book(&mut self, title: String) -> Result<(), LibraryError> {
      match self.books.iter_mut().find(|book| book.title == title) {
          Some(book) if !book.available => Err(LibraryError::AlreadyBorrowed),
          Some(book) => {
              book.available = false;
              Ok(())
          }
          None => Err(LibraryError::BookNotFound),
      }
  }
  
  fn op_return_book(&mut self, title: String) -> Result<(), LibraryError> {
      match self.books.iter_mut().find(|book| book.title == title) {
          Some(book) if book.available => Err(LibraryError::AlreadyAvailable),
          Some(book) => {
              book.available = true;
              Ok(())
          }
          None => Err(LibraryError::BookNotFound),
      }
  }
}


// Problem 2
enum Command {
  Add(f64, f64),
  Sub(f64, f64),
  Mul(f64, f64),
  Div(f64, f64)
}

fn execute(command: Command) -> Result<f64, String> {
  match command {
    Command::Add(a, b) => Ok(a + b),
    Command::Sub(a, b) => Ok(a - b),
    Command::Mul(a, b) => Ok(a * b),
    Command::Div(a, b) => {
      if b == 0.0 {
        Err(String::from("Zero division error"))
      } else {
        Ok(a / b)
      }
    },
  }
}

fn main() {
  match file_reader_simulator("config.txt".to_string()) {
    Ok(Some(data)) => println!("{}", data),
    Ok(None) => println!("File is empty"),
    Err(err) => println!("Error: {}", err)
  }
}
