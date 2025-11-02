fn main() {
    // Iterators
    let mut nums = vec![1, 2, 3, 4, 5];
    let nums_iter = nums.iter(); // Borrows each element
    for num in nums_iter {
      print!("{} ", num)
    }
    println!("");
    
    // Mutable iterators
    let nums_iter_mut = nums.iter_mut(); // Borrows mutable reference of each element, which means we need to make the original collection mutable
    for num in nums_iter_mut {
      *num = *num * 2;
    }
    println!("{:?}", nums);
    
    // Another way to iterate
    let mut nums_iter = nums.iter(); // Since calling next changes internal data of the iterator object, we need to make iterator variable mutable
    
    while let Some(val) = nums_iter.next() {
      print!("{} ", val);
    }
    println!("");
    
    // Into Iter
    let nums_iter = nums.into_iter(); // It will consume each element of collection instead of borrowing them, i.e. taking ownership of the collection
    
    for num in nums_iter {
      print!("{} ", num)
    }
    println!("");
    
    // nums; // Inaccessible since into_iter() will consume the entire collection
    // 
    // Bonus tip: Calling a normal for loop directly on a collection does the same thing as above, hence we loose the ownership of the collection after running the for loop.
    
    // Iterator Helpers
    // 1. Consuming Adapters
    // These are the methods that consumes (takes ownership) the iterator when it is called.
    // eg:- sum, min, max, map, etc
    let nums = vec![2, 4, 6, 8];
    println!("{}, {}", nums.iter().sum::<i32>(), nums.iter().min().unwrap_or(&0));
    
    // 2. Iterator Adaptors
    // These methods are defined in Iterator that returns a new iterator by changing some aspects of the original iterator without changing it.
    // Some iterator adaptors consumes the original iterator (like map, filter_map), some doesn't (filter).
    // eg: filter, etc.
    let nums = vec![2, 4, 6, 8];
    println!("{:?}", nums.iter().map(|num: &i32| num.pow(2)).collect::<Vec<i32>>());
    println!("{:?}", nums.iter().filter(|num| (*num) % 4 == 0).collect::<Vec<&i32>>());
}
