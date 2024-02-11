fn logical() {
    // Logical AND (&&)
    let a = true;
    let b = false;
    let result_and = a && b;
    println!("Logical AND: {}", result_and); // Output: false

    // Logical OR (||)
    let result_or = a || b;
    println!("Logical OR: {}", result_or); // Output: true

    // Logical NOT (!)
    let result_not_a = !a;
    let result_not_b = !b;
    println!("Logical NOT of a: {}", result_not_a); // Output: false
    println!("Logical NOT of b: {}", result_not_b); // Output: true
}
