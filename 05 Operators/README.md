# Operators

## Arithmetic Operators:-

In Rust, arithmetic operators are used to perform mathematical operations on numerical values. Here are the arithmetic operators in Rust along with examples:

- Addition: **`+`**

  Adds two values together.

```
    let sum = 5 + 3;
    println!("{}", sum); // Output: 8
```

- Subtraction: **`-`**

  Subtracts the right operand from the left operand.

```
    let difference = 10 - 4;
    println!("{}", difference); // Output: 6
```

- Multiplication: **`*`**

  Multiplies two values together.

```
    let product = 6 * 7;
    println!("{}", product); // Output: 42
```

- Division:**`/`**

  Divides the left operand by the right operand. Performs floating-point division.

```
    let quotient = 10.0 / 2.5;
    println!("{}", quotient); // Output: 4
```

- Remainder (Modulus): **`%`**

  Computes the remainder of the division of the left operand by the right operand.

```
    let remainder = 10 % 3;
    println!("{}", remainder); // Output: 1
```

- Unary Negation (Unary Minus): **`-`**

  Negates a single value.

```
    let negated_value = -5;
    println!("{}", negated_value); // Output: -5
```

These operators work with various numeric types including integers (signed and unsigned), floating-point numbers, and more.

## Comparison Operators

In Rust, comparison operators are used to compare values and produce boolean results (true or false). Here are the comparison operators in Rust:

- Equality: **`==`**

  Checks if
  two values are equal.

```
    let a = 5;
    let b = 5;
    println!("{}", a == b); // Output: true
```

- Inequality: **`!=`**

  Checks if
  two values are not equal.

```
    let a = 5;
    let b = 10;
    println!("{}", a != b); // Output: true
```

- Greater Than: **`>`**

  Checks if the left operand is greater than the right operand.

```
    let a = 10;
    let b = 5;
    println!("{}", a > b); // Output: true
```

- Less Than: **`<`**

  Checks if
  the left operand is less than the right operand.

```
    let a = 5;
    let b = 10;
    println!("{}", a < b); // Output: true
```

- Greater Than or Equal To: **`>=`**

  Checks if the left operand is greater than or equal to the right operand.

```
    let a = 10;
    let b = 10;
    println!("{}", a >= b); // Output: true
```

- Less Than or Equal To: **`<=`**

  Checks if the left operand is less than or equal to the right operand.

```
    let a = 5;
    let b = 10;
    println!("{}", a <= b); // Output: true
```

These operators can be used with various types such as integers, floating-point numbers, characters, and others for comparison in Rust.

## Logical Operators

In Rust, logical operators are used to perform logical operations on boolean values and produce boolean results. Here are the logical operators in Rust along with examples:

- Logical AND: **`&&`**

  Returns true if both operands are true, otherwise returns false.

```
    let a = true;
    let b = false;
    println!("{}", a && b); // Output: false
```

- Logical OR: **`||`**

  Returns true if at least one of the operands is true, otherwise returns false.

```
    let a = true;
    let b = false;
    println!("{}", a || b); // Output: true
```

- Logical NOT: **`!`**

  Reverses the boolean value of the operand.

```
    let a = true;
    println!("{}", !a); // Output: false
```

These operators are commonly used in conditional statements (if, while, etc.) and logical expressions to control the flow of execution in Rust programs.

## Assignment Operators

In Rust, assignment operators are used to assign values to variables and perform an operation on the variable at the same time. Here are some common assignment operators in Rust along with examples:

- Assignment **`(=)`**:

Assigns the value on the right-hand side to the variable on the left-hand side.

```let mut x = 5;
    x = 10;
    println!("{}", x); // Output: 10
```

- Addition Assignment **`(+=)`**:

Adds the value on the right-hand side to the variable on the left-hand side and assigns the result back to the variable.

```let mut x = 5;
    x += 3; // Equivalent to: x = x + 3;
    println!("{}", x); // Output: 8
```

- Subtraction Assignment **`(-=)`**:

Subtracts the value on the right-hand side from the variable on the left-hand side and assigns the result back to the variable.

```let mut x = 10;
    x -= 4; // Equivalent to: x = x - 4;
    println!("{}", x); // Output: 6
```

- Multiplication Assignment **`(*=)`**:

Multiplies the value on the right-hand side with the variable on the left-hand side and assigns the result back to the variable.

```let mut x = 3;
    x *= 2; // Equivalent to: x = x * 2;
    println!("{}", x); // Output: 6
```

- Division Assignment **`(/=)`**:

Divides the variable on the left-hand side by the value on the right-hand side and assigns the result back to the variable.

```let mut x = 10;
    x /= 2; // Equivalent to: x = x / 2;
    println!("{}", x); // Output: 5
```

- Modulus Assignment **`(%=)`**:

Computes the remainder of dividing the variable on the left-hand side by the value on the right-hand side and assigns the result back to the variable.

```let mut x = 10;
    x %= 3; // Equivalent to: x = x % 3;
    println!("{}", x); // Output: 1
```

These assignment operators are convenient shortcuts for performing arithmetic operations and updating variables in Rust.

## Bitwise Operators:

In Rust, bitwise operators are used to perform operations on individual bits of integer types. Here are the bitwise operators supported in Rust:

- Bitwise AND (&):

Performs a bitwise AND operation between each pair of corresponding bits.

```
    let result = 0b1010 & 0b1100; // 0b1000
    println!("{:b}", result); // Output: 1000
```

- Bitwise OR (|):

Performs a bitwise OR operation between each pair of corresponding bits.
```

    let result = 0b1010 | 0b1100; // 0b1110
    println!("{:b}", result); // Output: 1110
```

- Bitwise XOR (^):

Performs a bitwise XOR (exclusive OR) operation between each pair of corresponding bits.
```

    let result = 0b1010 ^ 0b1100; // 0b0110
    println!("{:b}", result); // Output: 0110
```

- Bitwise NOT (!):

Performs a bitwise NOT operation, which inverts each bit of the operand.
```

    let result = !0b1010; // 0b11111111111111111111111111110101
    println!("{:b}", result); // Output: 11111111111111111111111111110101
```

- Left Shift (<<):

Shifts the bits of the left operand to the left by a specified number of positions.

```
    let result = 0b1010 << 2; // 0b101000
    println!("{:b}", result); // Output: 101000
```

- Right Shift (>>):

Shifts the bits of the left operand to the right by a specified number of positions.

```
    let result = 0b1010 >> 2; // 0b10
    println!("{:b}", result); // Output: 10
```

These bitwise operators are commonly used in low-level programming, such as when dealing with hardware interactions, bit manipulation, or certain optimization techniques.

## Range Operator:

In Rust, the range operator is used to create ranges of values. There are two range operators:

- Exclusive Range **`(..)`**: Generates a range that includes the start value but excludes the end value.

- Inclusive Range **`(..=)`**: Generates a range that includes both the start and end values.

```
    // Exclusive Range
    fn main() {
        // Create a range from 1 to 5 (excluding 5)
        for num in 1..5 {
        println!("{}", num); // Output: 1, 2, 3, 4
        }
    }

    // Inclusive Range
    fn main() {
        // Create a range from 1 to 5 (including 5)
        for num in 1..=5 {
        println!("{}", num); // Output: 1, 2, 3, 4, 5
        }
    }
```