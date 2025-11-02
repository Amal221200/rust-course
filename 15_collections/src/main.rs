use std::collections::HashMap;


fn main() {
    // 1. Vector
    // let mut nums = vec![1, 2, 3];
    // nums.push(4);
    // nums.push(5);
    // nums.push(6);
    // nums.pop();
    // nums.insert(2, 10);
    // nums.sort();
    // nums.sort_by(|a, b| { b.cmp(a) });
    // println!("{:?}", nums);
    
    // 2. HashMaps
    let mut mapping: HashMap<&str, &str> = HashMap::new();
    mapping.insert("name", "Amal");
    mapping.insert("age", "90");
    mapping.remove("age");
    println!("{:?}", mapping);
}

fn group_values_by_key(pairs: Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
  let mut result: HashMap<String, Vec<i32>> = HashMap::new();
  for (key, value) in pairs {
    let data = result.get_mut(&key);
    match data {
      Some(val) => { val.push(value);},
      None => { result.insert(key, vec![value]); }
    }
  }
  
  // A little better approach
  // for (key, value) in pairs {
  //   if let Some(val) = result.get_mut(&key) {
  //     val.push(value);
  //   } else {
  //     result.insert(key, vec![value]);
  //   }
  // }
  
  // Better approach
  // for (key, value) in pairs {
  //   result.entry(key).or_insert(Vec::new()).push(value);
  // }
  return result;
}