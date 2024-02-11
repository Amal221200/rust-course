pub fn bitwise() {
    // Bitwise AND (&)
    let and_result = 0b1010 & 0b1100;
    println!("Bitwise AND Result: {:b}", and_result); // Output: 1000
    
    // Bitwise OR (|)
    let or_result = 0b1010 | 0b1100;
    println!("Bitwise OR Result: {:b}", or_result); // Output: 1110
    
    // Bitwise XOR (^)
    let xor_result = 0b1010 ^ 0b1100;
    println!("Bitwise XOR Result: {:b}", xor_result); // Output: 0110
    
    // Bitwise NOT (!)
    let not_result = !0b1010;
    println!("Bitwise NOT Result: {:b}", not_result); // Output: 11111111111111111111111111110101
    
    // Left Shift (<<)
    let left_shift_result = 0b1010 << 2;
    println!("Left Shift Result: {:b}", left_shift_result); // Output: 101000
    
    // Right Shift (>>)
    let right_shift_result = 0b1010 >> 2;
    println!("Right Shift Result: {:b}", right_shift_result); // Output: 10
}
