fn _demo() -> () {
    println!("Demo function")
}

fn _sum(a: i8, b: i8) {
    let sum = a + b;
    println!("The sum of {a} and {b} is: {sum}", a = a, b = b, sum = sum);
}

fn _sum1(a: i8, b: i8) -> i8 {
    return a + b;
}

fn _sum2(a: i8, b: i8) -> i8 {
    a + b
}

fn main() {
    // sum(34, 45);
    // let x = {
    //     let y = 56;
    //     y
    // };
    // println!("Hello World, {}", x);
}
