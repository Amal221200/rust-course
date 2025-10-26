## 1. Normal Struct
```
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
```

## 2. Tuple Struct
```
  #[derive(Debug)]
  struct RGB (u8, u8, u8);

  fn main() {
    let white = RGB (255, 255, 255);
    println!("White color {:?}", white);
    println!("Red {}, Blue {}, Green {}", white.0, white.1, white.2);
  }
```

## 3. Unit Struct
```
  struct Marker;

  fn main() {
    let marker = Marker;
  }
```