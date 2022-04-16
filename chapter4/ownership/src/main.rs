fn main() {
    return_value_scope()
}


// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
// 
// once a variable is out of scope, its value is dropped.
// scoping is managed by braces {}
// variables with a fixed sized and are immmutable are stored on then stack, those without are stored on the heap.

// for mutable variables with unknown size The memory must be requested from the memory allocator at runtime.
// We need a way of returning this memory to the allocator when we’re done with our String.
// memory is automatically returned once the variable that owns it goes out of scope.
// when reassining a variable with an unknown size, rust moves the pointer from the first variable to the other

//e.g let str1 = String::from("hello");
// let str2 = str1; here str1 is moved to str2 and str1 is dropped.
// println!("{}", str1); // this will cause an error

// when a move happens, rust does not deeply copy the data, it moves the pointer from one variable to the other.
// to perform a deep copy we need to use the clone method.

// eg  let str1 = String::from("hello");
// let str2 = str1.clone();
// println!("{}", str1); // this will print hello

// stack only data: copy
//let x = 5;
//let y = x;

//println!("x = {}, y = {}", x, y); this is valid becasue x and y are both stack allocated.
// Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack like integers are
// this means that the values of these types are copied when they are assigned to another variable.
// this is the default for primitive types.

// Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait.
// Here are some of the types that implement Copy:

 // All the integer types, such as u32.
 // The Boolean type, bool, with values true and false.
 // All the floating point types, such as f64.
 // The character type, char.
 // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

 fn ownership_flow() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.



// ************** Return Values and Scope *******************

fn return_value_scope() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string  // a_string is returned and moves out to the calling function
}


// References and Borrowing
// A reference is like a pointer in that it’s an address we can follow to access data stored at that address that is owned by some other variable.
// Because a refrence does not own the value , the value it points to will not be dropped when the reference stops being used.

//mutable refrences
fn main() {
  let mut s = String::from("hello");

  change(&mut s); //change fn can mutate s note the &mut s
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
// Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time. 

// let mut s = String::from("hello");

// let r1 = &mut s;
// let r2 = &mut s;

// println!("{}, {}", r1, r2); // errors out

// we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones

let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;


// we cant combine mutable and immutable refrences
//multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

//Dangling References
// if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

// The Rules of References
// - At any given time, you can have either one mutable reference or any number of immutable references.
// - References must always be valid.


// String Slices
// A string slice is a reference to part of a String, and it looks like this:
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

// With Rust’s .. range syntax, if you want to start at index zero, you can drop the value before the two periods. In other words, these are equal:

let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];

//  By the same token, if your slice includes the last byte of the String, you can drop the trailing number.
// You can also drop both values to take a slice of the entire string.

// Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

// string literals are immutable refrences
// slices of string literals and String values can be taken
// reference to &String are equivalent to whole slices of string literals