# Variables

Rust is a statically-typed language.

You can declare a variable using **``let``** keyword.

Example:
```
    // Type annotanion is optional for let
    let x: i32 = 5;
    println!("The value of x is {x}", x = x);
```

Rust variables are immutable by default, that means you cannot override the value of a variable. In order to do that, we need to change the variable to a mutable reference by adding the **``mut``** keyword after **``let``** .

Example:
```
    let mut x: i32 = 5;
    x = 10;
    println!("The value of x is {x}", x = x);
    // Output: The value of x is 10;
```
***Note: you cannot assign a value of a different type to a mutable reference.***

You can define a variable and declare its value later.

Example:
```
    let x;
    x = 10;
    println!("The value of x is {x}", x = x);
    // Output: The value of x is 10;
```
## Shadowing
Shadowing is a concept, which means you can redeclare the existing variable in a scope.

Example:
```
    let x: i32 = 5;
    let x = "John";
    println!("The value of x is {x}", x = x);
    // Output: The value of x is John;
```
***Note: you can assign a value of a different type in shadowing.***


## Constants
Constants is a type of variable where the value cannot be changed when its assinged. You can declared a constant using the **``const``** keyword.

Example:
```
    // Type annotation and value assignment is compulsory in constants
    const PI: i32 = 3.14;
    println!("The value of PI is {PI}", PI = PI);
    // Output: The value of PI is 3.14;
```

***Note: Constants once declared cannot be turned into a mutable reference or shadowed.***
