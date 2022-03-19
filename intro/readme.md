**Creating a project with cargo**
`cargo new hello_cargo`

use `cargo build` to build and generate executable

use `cargo run` to build and run program immediately

use `cargo check` to check if your code compiles without errors

***summary***
- We can build a project using cargo build.
- We can build and run a project in one step using cargo run.
- We can build a project without producing a binary to check for errors using cargo check.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
- When your project is finally ready for release, use cargo build --release to compile it with optimizations.
- `cargo doc --open` command will build documentation provided by all of your dependencies locally and open it in your browser.

**Guessing Game**

By default, Rust has a few items defined in the standard library that it brings into the scope of every program. This set is called the prelude, 
- to bring a type into scope explicitly (i.e its not in the prelude) we achieve this with a `use` statement.
e.g `use std::io;` i.e use `io` module from the `std` library
- declare a vvariable with `let` (immutable by default), to make it mutable use `let mut variable=value`
- `let mut guess = String::new()`, The `::` syntax in the `::new` line indicates that `new` is an associated function of the `String` type. 
-  An associated function is a function that’s implemented on a type, in this case String.
- io::stdin()
    .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in.
- The & indicates that this argument is a reference.
- reference gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
- variables, references are immutable by default.
-  you need to write &mut guess rather than &guess to make it mutable.
- `read_line` returns `io:Result`
- The Result types are enumerations,
- enums have a fixed set of possibilities known as variants.
-  Enums are often used with match, a conditional that makes it convenient to execute different code based on which variant an enum value is when the conditional is evaluated.
- Result’s variants are `Ok` or `Err`
- The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods. Chapter 10 will cover traits in detail.
