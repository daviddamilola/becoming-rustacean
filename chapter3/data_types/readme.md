data type subsets
- scalar
- compound

rust must know the types of all variables at compile time

The compiler can usually infer what type we want to use based on the value and how we use it

we must add a type annotation when multiple types are possible.

`let guess: u32 = "42".parse().expect("Not a number!");` different number types are possible here

***Scalar Types***
there are 4 primary scalar types
- integers
    * no fractional component
    * Each variant of integer type can be either signed or unsigned and has an explicit size.
    * Signed and unsigned refer to whether it’s possible for the number to be negative(signed) or not (unsigned)
    * Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses.
    * Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.
    * integer types default to i32.
    * When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics
    * instead it performs twos complement wrapping
    * values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. 
    * in u8, 256 becomes 0 , 257 becomes 1
    * To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

        -  Wrap in all modes with the wrapping_* methods, such as wrapping_add
        - Return the None value if there is overflow with the checked_* methods
        - Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
        - Saturate at the value’s minimum or maximum values with saturating_* methods


- floating-point numbers
    * Rust’s floating-point types are f32 (32 bits size) and f64 (64 bits size)
    * The default type is f64
    * Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.

*** numeric operations**
```
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}
```
- booleans
    * two main types are `true` and `false`
    * specified using the type `bool`
    * 1 byte in size

- characters
    * has the `char` type
    * `char` type is specified with single quotes
    * String literals use double quotes
    * 4 bytes in size and can represent values other than ascii
    * Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.


*** Compound Types ***
groups multiple values into one
 - tuples
    - Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    - We create a tuple by writing a comma-separated list of values inside parentheses
    - Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
     ```
        fn main() {
            let tup: (i32, f64, u8) = (500, 6.4, 1);
        }
    ```
    - tuples can be destructured e.g `let (x, y, z) = tup`
    - tuples can also be accessed with a dot followed by the index e.g `let x = tup.0; // first item in the tuple`
    - The tuple without any values, (), is a special type that has only one value, also written ()
    -  The type is called the unit type and the value is called the unit value. Expressions implicitly return the unit value if they don’t return any other value.
    
 - arrays
    - every element of an array must have the same type. 
    - arrays also have a fixed length
    - Arrays are useful when you want your data allocated on the stack rather than the heap
    - syntax : `let a: [i32; 5] = [1, 2, 3, 4, 5];`
    - A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
    - You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets. `let a = [3; 5];`
    - array elements are accesssed via indexing


Functions
 - Rust code uses snake case as the conventional style for function and variable names
 - functions argument must be typed
 - Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.
 - Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression
 - Expressions do not include ending semicolons.
 - If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
 - Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->).
 ```
    fn five() -> i32 {
        5
    }
 ```
 - the above example returns 5 because 5 there is an expression of i32 type, if we add a semi column to 5; it becomes a statement and the function returns the () unit value causing the code to throw an error


 Comments
  - single line comments //
  - multinple line requires adding single line comments to each line
  - there is also another type of comment called documentation comments.

  Control Flow
     - if expression and loops
     - If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop that we can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop.

     ```
        fn main() {
            let mut count = 0;
            'counting_up: loop {
                println!("count = {}", count);
                let mut remaining = 10;

                loop {
                    println!("remaining = {}", remaining);
                    if remaining == 9 {
                        break;
                    }
                    if count == 2 {
                        break 'counting_up;
                    }
                    remaining -= 1;
                }

                count += 1;
            }
            println!("End count = {}", count);
        }
     ```
