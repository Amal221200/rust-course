# Funtions

Fucntions are just a set of instructions that can be executed whenever it is called.

Syntax:

```
    fn fn_name(){
        // code
    }

    fn_name(); // Function call
```

Our main instructions (code) is always written inside the **_main_** function. Any code outside the **_main_** is not executed unless it is used inside the **_main_** function.

**_Note:- You can call the function before it's definition._**

## Parameters and Arguments

Parameters are the variable that stores the data that is passed to the function when it is called.

Arguments are the data that is passed to the function when it is called.

Syntax:-

```
    fn sum(a:i8, b:i8){ // a and b are parameters
        let sum = a+b;

        println!("The sum of {a} and {b} is: {sum}", a=a, b=b, sum=sum);
    }

    sum(12, 34); // 12 and 34 are arguments
```

**_Note:- 1. You can declare one or more parameters or no parameter at all._**

**_Note:- 2. Type of parameters needs to be defined explicitly._**

## Return

You can return a data from a function to a variable/storage where it is called.

There is two ways to return something from a function.

1. with **`return`** keyword

   ```
       fn sum(a:i8, b:i8) -> i8 { // -> means function return type signature
           let sum = a+b;

           return sum;
       }

       let a = 12;
       let b = 34
       let add = sum(a, b);
       println!("The sum of {a} and {b} is: {sum}", a=a, b=b, sum=sum);
       // Output "The sum of 12 and 34 is: 46"
   ```

2. without **`return`** keyword

   ```
       fn sum(a:i8, b:i8) -> i8 { // -> means function return type signature
           let sum = a+b;
           sum // in this expression the semi-colon is ommitted.
       }

       let a = 12;
       let b = 34
       let add = sum(a, b);
       println!("The sum of {a} and {b} is: {sum}", a=a, b=b, sum=sum);
       // Output "The sum of 12 and 34 is: 46"
   ```

**_Note:- In function definition, if the function returns a data, you have to define the return type of the funtion as you see in the above code using `->` symbol._**

**_Note:- In Rust, a function is an expression, i.e. it always returns something. It returns void by default if you don't return anything explicitly and it is denoted like this `()`._**
