# Data Types

Rust is a statically-typed language, which means that you need to explicitly tell the compiler what type of data the variable is storing.

Rust can infer the data type of a variable at runtime for most of the time.

Rust have 3 kinds of data types:-

1. **Scaler Type**: 
    1. Integer:- Any positive or negative numbers.
    2. Floating Point:- Any decimal numbers
    3. Boolean:- true/false
    4. Characters:- any single element inside ''.

2. **Compounds**:
    1. Tuples :- Tuple is a collection of data where data of each position is defined.

    ```
        // TUPLES
        let tup = (12, 213, "Amal");
    ```
    
    2. Arrays: :- Arrays is a homogenuos collection of data with fixed length.

    ```
        // ARRAYS
        let arr = [12, 213];
    ```

    ***Note:- We cannot mutate or change the content of arrays like in other language, for that we need to use a similar type called Vectors.***

    ***Note:- To print a compound type in Rust, you need to use this `{:?}` as a placeholder in the format string.***

    Example: 

    ```
        let arr = [12, 213];
        println!("{:?}", arr);
    ```

    ### Destructuring

    We can store each element of an array/tuple inside individual variables.
    Example:-
    ```
        // TUPLES
        let tup = (12, 213, "Amal");
        let (_x, _y, _z) = tup; // Destructing

        // ARRAYS
        let arr = [12, 213];
        let [_i, _j] = arr;
    ```

3. **Custom Types**:
    1. Structs :- Structs acts like a blueprint for a custom type which contain a key in string and value of any type.

        Example
        ```
            struct Person {
                name: String,
                age: u8,
            }

            let _person = Person {
                name: String::from("Amal Murikkoli"),
                age: 25,
            };

            println("The name is {} and the age is {}", person.name, person.age);
        ```
        ***Note:- It is almost like class and objects in other languages like C++, Java, etc.***

    2. Enums:- Enums is a custom data type that allows you to define a type by enumerating its possible values. Each of these values can be associated with different data.
    
    Example:-

    ```
        enum TraficLight {
            Red,
            Yellow,
            Green,
        }

        let light = TraficLight::Red;
    
        match light {
            TraficLight::Red => println!("Stop!"),
            TraficLight::Yellow => println!("Start!"),
            TraficLight::Green => println!("Go!"),
        }
    ```