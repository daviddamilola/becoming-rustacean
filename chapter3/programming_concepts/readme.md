**variables**
use `let` to declare variables

variables are immutable by default

to make variables mutable use `let mut variablename`


**constants**
Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

***Diffrences of `const` from `let`***
- you aren’t allowed to use `mut` with constants
- you must always annotate the type of a constant
- constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

Rust’s naming convention for constants is to use all uppercase with underscores between words.

The compiler is able to evaluate a limited set of operations at compile time,

Constants are valid for the entire time a program runs, within the scope they were declared in

**Shadowing**
you can declare a new variable with the same name as a previous variable.

the first variable is shadowed by the second (the second variable’s value is what the program sees when the variable is used.)

```
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

```
When the shadowing scope is over, the inner shadowing ends and x returns to being 6


