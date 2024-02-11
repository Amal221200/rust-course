fn assignment() {
    // Assignment (=)
    let mut x = 5;
    println!("x = {}", x); // Output: x = 5

    // Addition Assignment (+=)
    x += 3;
    println!("x += 3: {}", x); // Output: x += 3: 8

    // Subtraction Assignment (-=)
    x -= 2;
    println!("x -= 2: {}", x); // Output: x -= 2: 6

    // Multiplication Assignment (*=)
    x *= 2;
    println!("x *= 2: {}", x); // Output: x *= 2: 12

    // Division Assignment (/=)
    x /= 3;
    println!("x /= 3: {}", x); // Output: x /= 3: 4

    // Modulus Assignment (%=)
    x %= 3;
    println!("x %= 3: {}", x); // Output: x %= 3: 1

    // Bitwise AND Assignment (&=)
    let mut y = 0b1010;
    y &= 0b1100;
    println!("y &= 0b1100: {:b}", y); // Output: y &= 0b1100: 1000

    // Bitwise OR Assignment (|=)
    y |= 0b0110;
    println!("y |= 0b0110: {:b}", y); // Output: y |= 0b0110: 1110

    // Bitwise XOR Assignment (^=)
    y ^= 0b1010;
    println!("y ^= 0b1010: {:b}", y); // Output: y ^= 0b1010: 0100

    // Left Shift Assignment (<<=)
    let mut z = 0b1010;
    z <<= 2;
    println!("z <<= 2: {:b}", z); // Output: z <<= 2: 101000

    // Right Shift Assignment (>>=)
    z >>= 3;
    println!("z >>= 3: {:b}", z); // Output: z >>= 3: 101

}
