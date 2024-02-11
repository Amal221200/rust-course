pub fn range (){
    // Create a range from 1 to 5 (excluding 5)
    for num in 1..5 {
        println!("{}", num); // Output: 1, 2, 3, 4
    }

    // Create a range from 1 to 5 (including 5)
    for num in 1..=5 {
        println!("{}", num); // Output: 1, 2, 3, 4, 5
    }
}