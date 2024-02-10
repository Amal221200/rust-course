pub fn compound() {
    // TUPLES
    let tup = (12, 213, "Amal");
    let (_x, _y, _z) = tup; // Destructing

    // ARRAYS
    let arr = [12, 213];
    let [_i, _j] = arr;
    
    for num in arr.iter() {
        println!("{}", num);
    }
}
